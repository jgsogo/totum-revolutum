use anyhow::{anyhow, Result};
use std::future::Future;
use tokio::sync::oneshot::{channel, Receiver, Sender};
use tracing::error;

/// Spawns a new asynchronous task to execute the given `func` when the trigger signal is sent.
/// Using the return channel, the user can wait for the task to finish and receive the results.
pub fn create_side_task<F, Fut, Args, R>(func: F) -> (Sender<Args>, Receiver<Result<R>>)
where
    F: FnOnce(Args) -> Fut + Send + 'static,
    Args: Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    Fut::Output: Send + 'static,
{
    let (trigger, inner_wait) = channel();
    let (inner_done, on_completion) = channel();

    tokio::task::spawn(async move {
        let r = match inner_wait.await {
            Ok(args) => {
                let r = func(args).await;
                Ok(r)
            }
            Err(e) => Err(anyhow!("Error receiving the trigger signal: {e}")),
        };
        inner_done
            .send(r)
            .unwrap_or_else(|_| error!("Error sending done signal."));
    });

    (trigger, on_completion)
}
