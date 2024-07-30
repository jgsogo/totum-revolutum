use async_trait::async_trait;
use futures::Stream;

pub use map::PipelineStepMap;
pub use noop::PipelineStepNoop;
pub use syncronize::{PipelineStepSyncronizeEnd, PipelineStepSyncronizeStart};

use crate::{Message, PipelineData};

mod map;
mod noop;
mod stop_on_error;
mod syncronize;

#[async_trait]
pub trait PipelineStepAsync<Input: PipelineData, Output: PipelineData>: Send + 'static {
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: flume::Sender<Message<Output>>);
}

#[cfg(test)]
pub(crate) mod tests {
    use tokio_stream::StreamExt;

    use crate::{Message, PipelineData};

    pub(crate) async fn collect_rx<T: PipelineData>(rx: flume::Receiver<Message<T>>) -> Vec<T> {
        rx.into_stream()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|v| match v {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>()
    }
}
