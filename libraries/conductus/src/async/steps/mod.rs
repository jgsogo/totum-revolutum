mod noop;

use crate::sync::{Message, PipelineData};
use async_trait::async_trait;
use futures::Stream;
pub use noop::PipelineStepNoop;

#[async_trait]
pub trait PipelineStep<Input: PipelineData, Output: PipelineData>: Send + 'static {
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: flume::Sender<Message<Output>>);
}
