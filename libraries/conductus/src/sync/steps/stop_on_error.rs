use std::marker::PhantomData;

use flume::Sender;
use tracing::debug;

use crate::sync::steps::PipelineStepSync;
use crate::{Message, PipelineData};

/// A [`PipelineStepSync`] that can be used to stop a pipeline for certain errors. Return `Some(reason)`
/// to stop the pipeline, or `None` to ignore and skip the error.
pub struct PipelineStepStopOnError<Func, Error: std::error::Error>
where
    Func: Fn(Error) -> Option<String>,
{
    func: Func,
    _error: PhantomData<Error>,
}

impl<Func, Error: std::error::Error> PipelineStepStopOnError<Func, Error>
where
    Func: Fn(Error) -> Option<String>,
{
    pub fn check_error(&self, err: Error) -> Option<String> {
        (self.func)(err)
    }
}

impl<Input: PipelineData, Func, Error: std::error::Error + Send + 'static> PipelineStepSync<Result<Input, Error>, Input>
    for PipelineStepStopOnError<Func, Error>
where
    Func: Fn(Error) -> Option<String> + Send + 'static,
{
    fn run<I: Iterator<Item = Result<Input, Error>>>(&self, source: I, target: Sender<Message<Input>>) {
        for it in source {
            let msg = match it {
                Ok(v) => Some(Message::Data(v)),
                Err(e) => self.check_error(e).map(Message::Stop),
            };

            if let Some(msg) = msg {
                if let Err(e) = target.send(msg) {
                    debug!("Error sending from blanket implementation of PipelineStepStopOnError: {e}");
                }
            }
        }
    }
}

impl<Func, Error: std::error::Error> From<Func> for PipelineStepStopOnError<Func, Error>
where
    Func: Fn(Error) -> Option<String>,
{
    fn from(value: Func) -> Self {
        PipelineStepStopOnError {
            func: value,
            _error: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display, Formatter};

    use crate::sync::steps::tests::collect_rx;

    use super::*;

    struct MyError(i32);

    impl Debug for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError({})", self.0)
        }
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyError({})", self.0)
        }
    }

    impl std::error::Error for MyError {}

    #[test]
    fn test_step_stop_on_error() {
        let stop_on_error = |input: MyError| {
            if input.0 == 3 {
                Some("Stop because error was 3".to_string())
            } else if input.0 == 5 {
                Some("Stop because error was 5".to_string())
            } else {
                None
            }
        };
        let step: PipelineStepStopOnError<_, MyError> = stop_on_error.into();

        assert!(step.check_error(MyError(2)).is_none());
        assert!(step.check_error(MyError(3)).is_some());
        assert!(step.check_error(MyError(5)).is_some());

        let (tx, rx) = flume::bounded::<Message<i32>>(2);
        let data = vec![Err(MyError(0)), Ok(1), Ok(2), Err(MyError(3))];
        std::thread::spawn(move || step.run(data.into_iter(), tx));

        let r = collect_rx(rx);
        assert_eq!(r, vec![1, 2])
    }
}
