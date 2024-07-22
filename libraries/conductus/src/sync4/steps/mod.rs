pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;
pub use sync::{PipelineStepSyncEnd, PipelineStepSyncStart, SyncMarked};
pub use window::PipelineStepWindow;

use super::pipeline::Message;

mod map;
mod noop;
mod sync;
mod window;

pub trait PipelineStep<Input, Output> {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}
