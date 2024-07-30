use crate::{Message, PipelineData, PipelineHeadImpl};
use async_trait::async_trait;
use flume::SendError;
use futures::{pin_mut, Stream};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

/// Interface for everything that can act as the head of a pipeline
#[async_trait]
pub trait PipelineHeadAsync {
    type TInput: PipelineData;

    /// Sends one item into the head of the pipeline. This is a blocking call.
    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    /// Sends several items into the head of the pipeline.
    ///
    /// Implementors should detach the caller from the actual send into the channels.
    async fn send_batch<I: Stream<Item = Self::TInput> + Send>(&self, input: I) -> Result<(), SendError<Self::TInput>> {
        pin_mut!(input);
        while let Some(d) = input.next().await {
            self.send(d).await?;
        }
        Ok(())
    }

    async fn send_detached<I: Stream<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>;
}

#[async_trait]
impl<Input: PipelineData> PipelineHeadAsync for PipelineHeadImpl<Input> {
    type TInput = Input;

    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.tx.send_async(Message::Data(item)).await.map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    async fn send_detached<I: Stream<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            pin_mut!(input);
            while let Some(d) = input.next().await {
                tx.send_async(Message::Data(d)).await.map_err(|e| {
                    let Message::Data(msg) = e.into_inner() else {
                        unreachable!()
                    };
                    SendError(msg)
                })?
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use futures::{stream, StreamExt};

    use super::*;

    async fn collect_results<T>(rx: flume::Receiver<T>) -> Vec<T> {
        let mut received = Vec::new();
        let mut stream = rx.into_stream();
        while let Some(value) = stream.next().await {
            received.push(value);
        }
        received
    }

    #[tokio::test]
    async fn test_send() {
        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        head.send(10).await.unwrap();
        head.send(42).await.unwrap();
        drop(head);

        // Collect the results
        let received = collect_results(rx).await;
        assert_eq!(received, vec![Message::Data(10), Message::Data(42),])
    }

    #[tokio::test]
    async fn test_send_batch() {
        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        head.send_batch(stream::iter(1..3)).await.unwrap();
        drop(head);

        // Collect the results
        let received = collect_results(rx).await;
        assert_eq!(received, vec![Message::Data(1), Message::Data(2),])
    }

    #[tokio::test]
    async fn test_send_detached() {
        let (tx, rx) = flume::bounded(2);
        let head = PipelineHeadImpl::new(tx);

        head.send_detached(stream::iter(1..5)).await;
        drop(head);

        // Collect the results
        let received = collect_results(rx).await;
        assert_eq!(
            received,
            vec![Message::Data(1), Message::Data(2), Message::Data(3), Message::Data(4),]
        )
    }
}
