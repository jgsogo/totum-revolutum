use async_trait::async_trait;
use futures::Stream;

use crate::{Message, PipelineData};
pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;

mod map;
mod noop;

#[async_trait]
pub trait PipelineStepAsync<Input: PipelineData, Output: PipelineData>: Send + 'static {
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: flume::Sender<Message<Output>>);
}
