use crate::{Message, PipelineData};
pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;
pub use stop_on_error::PipelineStepStopOnError;
pub use sync::{PipelineStepSyncEnd, PipelineStepSyncStart, SyncMarked, SyncMarkedTrait};
pub use window::PipelineStepWindow;

mod map;
mod noop;
mod stop_on_error;
mod sync;
mod window;

/// Interface for all the steps in the `conductus` library
pub trait PipelineStep<Input: PipelineData, Output: PipelineData>: Send + 'static {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}
