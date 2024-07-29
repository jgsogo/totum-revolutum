use async_trait::async_trait;
use flume::Sender;
use futures::{pin_mut, Stream};
use tokio_stream::StreamExt;
use tracing::debug;

use crate::sync::{Message, PipelineData};

use super::PipelineStep;

/// A [`PipelineStep`] that does nothing
#[derive(Default)]
pub struct PipelineStepNoop;

#[async_trait]
impl<Input: PipelineData> PipelineStep<Input, Input> for PipelineStepNoop {
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: Sender<Message<Input>>) {
        pin_mut!(source);
        while let Some(it) = source.next().await {
            if let Err(e) = target.send_async(Message::Data(it)).await {
                debug!("Error sending from blanket implementation of PipelineStepBuffer: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{stream, StreamExt};

    #[tokio::test]
    async fn test_step_buffer() {
        let step = PipelineStepNoop::default();

        let (tx, rx) = flume::bounded(2);
        tokio::spawn(async move {
            step.run(stream::iter(0..10), tx).await;
        });

        let r = rx
            .into_stream()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|v| match v {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
