use crate::sync::PipelineData;
use async_trait::async_trait;
use flume::SendError;

/// Interface for everything that can act as the head of a pipeline
#[async_trait]
pub trait PipelineHead {
    type TInput: PipelineData;

    /// Sends one item into the head of the pipeline. This is a blocking call.
    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    /// Sends several items into the head of the pipeline.
    ///
    /// Implementors should detach the caller from the actual send into the channels.
    async fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> tokio::task::JoinHandle<Result<(), SendError<Self::TInput>>>
    where
        <I as IntoIterator>::IntoIter: Send;
}
