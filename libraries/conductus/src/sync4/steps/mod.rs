mod map;

pub trait PipelineStep<Input, Output> {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}

use super::pipeline::Message;
pub use map::PipelineStepMap;
