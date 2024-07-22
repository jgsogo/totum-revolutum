pub use map::PipelineStepMap;
pub use window::PipelineStepWindow;

use super::pipeline::Message;

mod map;
mod window;

pub trait PipelineStep<Input, Output> {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}
