use std::future::Future;
use std::marker::PhantomData;

use crate::{Error, Result};
use log::{debug, error};
use tokio::sync::oneshot::{channel, Receiver, Sender};
use tokio::task::JoinHandle;

/// Implements RAII pattern. This object stores a future that is guaranteed to be executed:
/// The user can decide when the task is started:
///  * immediately: use [`SideTask::new_and_start`],
///  * at certain point in time: (create with [`SideTask::new`] and use [`SideTask::start`] to
///    start), or
///  * when it's about to drop (just create with [`SideTask::new`]).
///
/// When the object is created, it also returns a [`Receiver`] that the user can use to await
/// the result from the future. Alternatively, the user can await on the [`JoinHandle`] returned
/// from the [`SideTask::start`] function (result is not returned).
///
/// **To guarantee that the task is fully executed, the user needs to await either the
/// [`JoinHandle`] or the [`Receiver`]**, otherwise, it's up to the user to ensure that the
/// tokio runtime lives long enough to finish the task.
///
/// TODO: Can we execute the task in [`Drop::drop`] and wait for it completion? Is this related to
/// TODO: the "Async Drop" discussion in Rust internet? Would something like this work...?
/// TODO: ```
/// TODO: let runtime = tokio::runtime::Runtime::new().unwrap();
/// TODO: let s = runtime.block_on(...async function...)
/// TODO: ```
///
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
    /// Creates a [`SideTask`] object, but doesn't start the underlying task yet. Use the returned
    /// object to start the task with [`SideTask::start`] and the [`Receiver`] to await for the
    /// task result. If the task is not started manually, it will be triggered on [`Drop`].
    pub fn new(task: F, args: Option<Args>) -> (Self, Receiver<R>) {
        //let (trigger, inner_rx) = channel();
        let (inner_tx, inner_rx) = channel();

        let s = Self {
            task: Some(task),
            args,

            inner_tx: Some(inner_tx),
            _fut: Default::default(),
            _args: Default::default(),
        };
        (s, inner_rx)
    }

    /// Creates a [`SideTask`] object and starts the task immediately. Use the returned [`Receiver`]
    /// to await for the task result.
    pub fn new_and_start(func: F, args: Args) -> Result<(Self, Receiver<R>)> {
        let (mut s, receiver) = Self::new(func, None);
        s.start(Some(args))?;
        Ok((s, receiver))
    }

    /// Start the underlying task, if it hasn't been started yet. The task will be started with
    /// the given arguments if they were not already provided when the [`SideTask`] was created.
    pub fn start(&mut self, args: Option<Args>) -> Result<JoinHandle<()>> {
        let task = self.task.take().ok_or(Error::TaskIsAlreadyRunning)?;
        let args = match (args, self.args.take()) {
            (Some(_input), Some(_stored)) => {
                return Err(Error::DuplicatedArguments);
            }
            (None, None) => return Err(Error::MissingArguments),
            (Some(input), _) => input,
            (_, Some(stored)) => stored,
        };
        let inner_tx = self.inner_tx.take().unwrap();

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
    /// Drop the [`SideTask`] object. If the underlying task hasn't been started yet, it will be
    /// triggered now. It's the responsibility of the user to wait until the task finishes before
    /// shutting-down the current runtime (the user can await on the returned [`Receiver`] when the
    /// `SideTask` was created or started.
    fn drop(&mut self) {
        // If the task didn't start yet, trigger it now (with stored arguments)
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
    use anyhow::anyhow;
    use log::{info, Level};
    use std::time::Duration;
    use testing_logger;
    use tokio::time::sleep;

    async fn task(value: i32) -> i32 {
        info!("Task executed with value {value}");
        value * 2
    }
    async fn task_return_error(_value: i32) -> anyhow::Result<()> {
        info!("Task fail: execute");
        Err(anyhow!("Error from task"))
    }

    #[tokio::test]
    async fn new_then_start_with_arguments() -> Result<()> {
        testing_logger::setup();

        let (mut task, receiver) = SideTask::new(task, None);
        task.start(Some(24i32))?;

        let value = receiver.await.unwrap();
        assert_eq!(value, 48i32);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task executed with value 24");
            assert_eq!(captured_logs[0].level, Level::Info);
        });
        Ok(())
    }

    #[tokio::test]
    async fn new_with_arguments_then_start() -> Result<()> {
        testing_logger::setup();

        let (mut task, receiver) = SideTask::new(task, Some(10));
        task.start(None)?;

        let value = receiver.await.unwrap();
        assert_eq!(value, 20);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task executed with value 10");
            assert_eq!(captured_logs[0].level, Level::Info);
        });
        Ok(())
    }

    #[tokio::test]
    async fn new_and_start() {
        testing_logger::setup();

        let (_, receiver) = SideTask::new_and_start(task, 20).unwrap();
        let value = receiver.await.unwrap();
        assert_eq!(value, 40);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task executed with value 20");
            assert_eq!(captured_logs[0].level, Level::Info);
        });
    }

    #[tokio::test]
    async fn new_and_drop() {
        testing_logger::setup();

        // Variable `_` ignores the output and it's immediately dropped
        let (_, receiver) = SideTask::new(task, Some(30));
        let value = receiver.await.unwrap();
        assert_eq!(value, 60);

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task executed with value 30");
            assert_eq!(captured_logs[0].level, Level::Info);
        });
    }

    #[tokio::test]
    async fn duplicated_args() {
        let (mut task, _) = SideTask::new(task, Some(40));
        let r = task.start(Some(50));
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), Error::DuplicatedArguments));
    }

    #[tokio::test]
    async fn no_args() {
        let (mut task, _) = SideTask::new(task, None);
        let r = task.start(None);
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), Error::MissingArguments));
    }

    #[tokio::test]
    async fn multiple_start() {
        let (mut task, _) = SideTask::new(task, Some(50));
        task.start(None).unwrap();
        let r = task.start(None);
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), Error::TaskIsAlreadyRunning));
    }

    #[tokio::test]
    async fn receiver_dropped() {
        testing_logger::setup();

        // Variable `_` ignores the output and it's immediately dropped
        let (_, _) = SideTask::new(task, Some(60));
        // We need to sleep so the task has time to finish
        sleep(Duration::from_millis(200)).await;

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Task executed with value 60");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(
                captured_logs[1].body,
                "Error sending the result from inside the task. The receiver might be dropped"
            );
            assert_eq!(captured_logs[1].level, Level::Debug);
        });
    }

    #[tokio::test]
    async fn receiver_dropped_but_thread_joined() -> anyhow::Result<()> {
        let (mut task, _) = SideTask::new(task, Some(110));
        let j = task.start(None)?;
        // Wait for the thread to join. Task is finished.
        j.await?;

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 2);
            assert_eq!(captured_logs[0].body, "Task executed with value 110");
            assert_eq!(captured_logs[0].level, Level::Info);
            assert_eq!(
                captured_logs[1].body,
                "Error sending the result from inside the task. The receiver might be dropped"
            );
            assert_eq!(captured_logs[1].level, Level::Debug);
        });

        Ok(())
    }

    #[tokio::test]
    async fn task_that_fails() -> anyhow::Result<()> {
        testing_logger::setup();

        let (_, receiver) = SideTask::new_and_start(task_return_error, 1)?;
        let value = receiver.await?;
        assert!(value.is_err());
        assert_eq!(value.unwrap_err().to_string(), "Error from task");

        testing_logger::validate(|captured_logs| {
            assert_eq!(captured_logs.len(), 1);
            assert_eq!(captured_logs[0].body, "Task fail: execute");
            assert_eq!(captured_logs[0].level, Level::Info);
        });

        Ok(())
    }
}
