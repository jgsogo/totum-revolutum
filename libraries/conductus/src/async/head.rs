use crate::sync::PipelineData;
use async_trait::async_trait;
use flume::SendError;
use tokio::task::JoinHandle;

/// Interface for everything that can act as the head of a pipeline
#[async_trait]
pub trait PipelineHead {
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
