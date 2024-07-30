use std::future::Future;
use std::marker::PhantomData;

use async_trait::async_trait;
use flume::Sender;
use futures::{pin_mut, Stream};
use tokio_stream::StreamExt;
use tracing::debug;

use crate::{Message, PipelineData};

use super::PipelineStepAsync;

pub struct PipelineStepStopOnError<Fut, F, Error: std::error::Error>
where
    F: Fn(Error) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<String>> + Send + 'static,
{
    func: F,
    _fut: PhantomData<Fut>,
    _error: PhantomData<Error>,
}

impl<Fut, F, Error: std::error::Error> PipelineStepStopOnError<Fut, F, Error>
where
    F: Fn(Error) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<String>> + Send + 'static,
{
    pub async fn check_error(&self, input: Error) -> Option<String> {
        (self.func)(input).await
    }
}

#[async_trait]
impl<Input: PipelineData + Sync, Fut, F, Error: std::error::Error + Send + Sync + 'static>
    PipelineStepAsync<Result<Input, Error>, Input> for PipelineStepStopOnError<Fut, F, Error>
where
    F: Fn(Error) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<String>> + Send + Sync + 'static,
{
    async fn run<I: Stream<Item = Result<Input, Error>> + Send>(&self, source: I, target: Sender<Message<Input>>) {
        pin_mut!(source);
        while let Some(it) = source.next().await {
            let msg = match it {
                Ok(v) => Some(Message::Data(v)),
                Err(e) => self.check_error(e).await.map(Message::Stop),
            };

            if let Some(msg) = msg {
                if let Err(e) = target.send_async(msg).await {
                    debug!("Error sending from blanket implementation of PipelineStepStopOnError: {e}");
                }
            }
        }
    }
}

impl<Fut, F, Error: std::error::Error> From<F> for PipelineStepStopOnError<Fut, F, Error>
where
    F: Fn(Error) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<String>> + Send + 'static,
{
    fn from(value: F) -> Self {
        Self {
            func: value,
            _fut: PhantomData,
            _error: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Debug, Display, Formatter};

    use futures::stream;

    use crate::r#async::steps::tests::collect_rx;

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

    #[tokio::test]
    async fn test_step_stop_on_error() {
        let step = PipelineStepStopOnError::from(move |input: MyError| async move {
            if input.0 == 3 {
                Some("Stop because error was 3".to_string())
            } else if input.0 == 5 {
                Some("Stop because error was 5".to_string())
            } else {
                None
            }
        });

        assert!(step.check_error(MyError(0)).await.is_none());
        assert!(step.check_error(MyError(3)).await.is_some());
        assert!(step.check_error(MyError(5)).await.is_some());

        let (tx, rx) = flume::bounded(2);
        let data = vec![Err(MyError(0)), Ok(1), Ok(2), Err(MyError(3))];
        tokio::spawn(async move { step.run(stream::iter(data), tx).await });

        let r = collect_rx(rx).await;
        assert_eq!(r, vec![1, 2])
    }
}
