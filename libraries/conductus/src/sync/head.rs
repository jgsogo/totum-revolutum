use crate::PipelineData;
use flume::SendError;
use std::thread::JoinHandle;

/// Interface for everything that can act as the head of a pipeline
pub trait PipelineHead {
    type TInput: PipelineData;

    /// Sends one item into the head of the pipeline. This is a blocking call.
    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    /// Sends several items into the head of the pipeline.
    ///
    /// Implementors should detach the caller from the actual send into the channels.
    fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>;
}
