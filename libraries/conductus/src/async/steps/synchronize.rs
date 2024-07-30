use std::collections::BinaryHeap;

use async_trait::async_trait;
use flume::Sender;
use futures::{pin_mut, Stream, StreamExt};
use tracing::debug;

use crate::common::steps::synchronize::SynchronizeMarkedTrait;
use crate::common::steps::synchronize::{PipelineStepSynchronizeEnd, PipelineStepSynchronizeStart, SynchronizeMarked};
use crate::r#async::steps::PipelineStepAsync;
use crate::{Message, PipelineData};

#[async_trait]
impl<Input: PipelineData> PipelineStepAsync<Input, SynchronizeMarked<Input>> for PipelineStepSynchronizeStart {
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: Sender<Message<SynchronizeMarked<Input>>>) {
        let source = source.enumerate();

        pin_mut!(source);
        while let Some((i, it)) = source.next().await {
            if let Err(e) = target.send_async(Message::Data(SynchronizeMarked::new(i, it))).await {
                debug!("Error sending from implementation of PipelineStepSynchronizeStart: {e}");
            }
        }
    }
}

#[async_trait]
impl<InnerInput: PipelineData, Input: SynchronizeMarkedTrait<InnerInput>> PipelineStepAsync<Input, InnerInput>
    for PipelineStepSynchronizeEnd
{
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: Sender<Message<InnerInput>>) {
        let mut next = 0;
        let mut heap = BinaryHeap::new();
        pin_mut!(source);
        while let Some(it) = source.next().await {
            heap.push(it);

            while let Some(peek) = heap.peek() {
                if peek.mark() == next {
                    let item = heap.pop().expect("Already checked above");
                    if let Err(e) = target.send_async(Message::Data(item.into_inner())).await {
                        debug!("Error sending from implementation of PipelineStepSynchronizeEnd: {e}");
                    }
                    next += 1;
                } else {
                    assert!(peek.mark() > next);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::stream;

    use crate::r#async::steps::tests::collect_rx;

    use super::*;

    #[tokio::test]
    async fn test_synchronize_step_start() {
        let sync_start = PipelineStepSynchronizeStart;

        let (tx, rx) = flume::bounded(2);
        tokio::spawn(async move { sync_start.run(stream::iter(10..12), tx).await });

        let r = collect_rx(rx).await;
        assert_eq!(r, vec![SynchronizeMarked::new(0, 10), SynchronizeMarked::new(1, 11)]);
    }

    #[tokio::test]
    async fn test_synchronize_step_end() {
        let sync_end = PipelineStepSynchronizeEnd;

        let (tx, rx) = flume::bounded(2);
        let data = vec![
            SynchronizeMarked::new(0, 10),
            SynchronizeMarked::new(2, 12),
            SynchronizeMarked::new(1, 11),
        ];
        tokio::spawn(async move { sync_end.run(stream::iter(data.into_iter()), tx).await });

        let r = collect_rx(rx).await;
        assert_eq!(r, vec![10, 11, 12]);
    }
}
