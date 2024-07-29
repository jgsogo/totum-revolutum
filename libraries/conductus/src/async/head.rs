use crate::{Message, PipelineData, PipelineHeadImpl};
use async_trait::async_trait;
use flume::SendError;
use tokio::task::JoinHandle;

/// Interface for everything that can act as the head of a pipeline
#[async_trait]
pub trait PipelineHeadAsync {
    type TInput: PipelineData;

    /// Sends one item into the head of the pipeline. This is a blocking call.
    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    /// Sends several items into the head of the pipeline.
    ///
    /// Implementors should detach the caller from the actual send into the channels.
    async fn send_batch<I: IntoIterator<Item = Self::TInput> + Send>(
        &self,
        input: I,
    ) -> Result<(), SendError<Self::TInput>>
    where
        <I as IntoIterator>::IntoIter: Send;

    async fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>
    where
        <I as IntoIterator>::IntoIter: Send;
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

    async fn send_batch<I: IntoIterator<Item = Self::TInput> + Send>(&self, input: I) -> Result<(), SendError<Input>>
    where
        <I as IntoIterator>::IntoIter: Send,
    {
        for it in input {
            self.send(it).await?;
        }
        Ok(())
    }

    async fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>
    where
        <I as IntoIterator>::IntoIter: Send,
    {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            for it in input {
                tx.send_async(Message::Data(it)).await.map_err(|e| {
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
    use futures::StreamExt;

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

        head.send_batch(1..3).await.unwrap();
        drop(head);

        // Collect the results
        let received = collect_results(rx).await;
        assert_eq!(received, vec![Message::Data(1), Message::Data(2),])
    }

    #[tokio::test]
    async fn test_send_detached() {
        let (tx, rx) = flume::bounded(2);
        let head = PipelineHeadImpl::new(tx);

        head.send_detached(1..5).await;
        drop(head);

        // Collect the results
        let received = collect_results(rx).await;
        assert_eq!(
            received,
            vec![Message::Data(1), Message::Data(2), Message::Data(3), Message::Data(4),]
        )
    }
}
