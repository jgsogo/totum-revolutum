pub use map::PipelineStepMap;
pub use stop_on_error::PipelineStepStopOnError;
pub use window::PipelineStepWindow;

pub use crate::common::steps::syncronize::SyncronizeMarkedTrait;
use crate::{Message, PipelineData};

mod map;
mod noop;
mod stop_on_error;
mod syncronize;
mod window;

/// Interface for all the steps in the `conductus` library
pub trait PipelineStepSync<Input: PipelineData, Output: PipelineData>: Send + 'static {
    fn run<I: Iterator<Item = Input>>(&self, source: I, target: flume::Sender<Message<Output>>);
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::{Message, PipelineData};

    pub(crate) fn collect_rx<T: PipelineData>(rx: flume::Receiver<Message<T>>) -> Vec<T> {
        rx.into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>()
    }
}
