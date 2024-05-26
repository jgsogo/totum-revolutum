use std::future::Future;
use std::marker::PhantomData;

use anyhow::{anyhow, bail, Result};
use log::{debug, error};
use tokio::sync::oneshot::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

/// Implements RAII pattern. This object stores a future that is guaranteed to be executed:
/// The user can decide when the task is started:
///  * immediately: use `SideTask::new_and_start`,
///  * at certain point in time: (create with `SideTask::new` and use `SideTask::start` to start), or
///  * when it's about to drop (just create with `SideTask::new`).
///
/// When the object is created, it also returns a `Receiver` that the user can use to receive
/// the result from the future. Only awaiting this receiver it's guaranteed that the future will
/// finish it's execution, otherwise the program might end before the future is completed.
///
/// Alternatively, if `SideTask::start` is used, the user can wait on the returned JoinHandle
pub struct SideTask<F, Fut, Args, R>
where
    F: FnOnce(Args) -> Fut + Send + 'static,
    Args: Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    Fut::Output: Send + 'static,
{
    task: Option<F>,
    args: Option<Args>,

    inner_tx: Option<Sender<R>>,

    //
    _fut: PhantomData<Fut>,
    _args: PhantomData<Args>,
}

impl<F, Fut, Args, R> SideTask<F, Fut, Args, R>
where
    F: FnOnce(Args) -> Fut + Send + 'static,
    Args: Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    Fut::Output: Send + 'static,
{
    pub fn new(task: F, args: Option<Args>) -> Result<(Self, Receiver<R>)> {
        //let (trigger, inner_rx) = channel();
        let (inner_tx, inner_rx) = channel();

        let s = Self {
            task: Some(task),
            args,

            inner_tx: Some(inner_tx),
            _fut: Default::default(),
            _args: Default::default(),
        };
        Ok((s, inner_rx))
    }

    pub fn new_and_start(func: F, args: Args) -> Result<(Self, Receiver<R>)> {
        let (mut s, receiver) = Self::new(func, None)?;
        s.start(Some(args))?;
        Ok((s, receiver))
    }

    pub fn start(&mut self, args: Option<Args>) -> Result<JoinHandle<()>> {
        let task = self.task.take().ok_or(anyhow!("Error taking task"))?;
        let args = match (args, self.args.take()) {
            (Some(_input), Some(_stored)) => {
                bail!("Arguments were already provided when the object was created")
            }
            (None, None) => bail!("Arguments need to be provided when the object is created or now in the start call"),
            (Some(input), _) => input,
            (_, Some(stored)) => stored,
        };
        let inner_tx = self.inner_tx.take().ok_or(anyhow!("Error taking 'inner_tx'"))?;

        let join_handle = tokio::task::spawn(async move {
            let r = task(args).await;
            inner_tx.send(r).unwrap_or_else(|_| {
                debug!("Error sending the result from inside the task. The receiver might be dropped")
            });
        });
        Ok(join_handle)
    }
}

impl<F, Fut, Args, R> Drop for SideTask<F, Fut, Args, R>
where
    F: FnOnce(Args) -> Fut + Send + 'static,
    Args: Send + 'static,
    Fut: Future<Output = R> + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn drop(&mut self) {
        // If the task didn't started yet, trigger it now (with stored arguments)
        if self.task.is_some() {
            if let Err(e) = self.start(None) {
                error!("Error starting the task from Drop: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{info, Level};
    use testing_logger;

    async fn task(value: i32) -> i32 {
        info!("Task executed with value {value}");
        value * 2
    }

    #[tokio::test]
    async fn test_new() -> Result<()> {
        testing_logger::setup();

        let (mut task, receiver) = SideTask::new(task, None)?;
        task.start(Some(24i32))?;
        // drop(task);

        let value = receiver.await?;
        assert_eq!(value, 48i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task executed with value 24");
            assert_eq!(captured_logs[0].level, Level::Info);
        });

        Ok(())
    }
}
