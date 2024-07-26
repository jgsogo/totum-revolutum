use crate::sync::PipelineData;
use flume::SendError;
use std::thread::JoinHandle;

pub trait PipelineHead {
    type TInput: PipelineData;

    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>;
}
