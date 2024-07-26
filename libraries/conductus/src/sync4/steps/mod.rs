use crate::sync4::Message;
pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;
pub use sync::{PipelineStepSyncEnd, PipelineStepSyncStart, SyncMarked, SyncMarkedTrait};
pub use window::PipelineStepWindow;

mod map;
mod noop;
mod sync;
mod window;

pub trait PipelineStep<Input, Output> {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}
