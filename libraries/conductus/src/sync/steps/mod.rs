pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;
pub use sync::{PipelineStepSyncEnd, PipelineStepSyncStart, SyncMarked, SyncMarkedTrait};
pub use window::PipelineStepWindow;

use crate::sync::{Message, PipelineData};

mod map;
mod noop;
mod sync;
mod window;

pub trait PipelineStep<Input: PipelineData, Output: PipelineData>: Send + 'static {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}
