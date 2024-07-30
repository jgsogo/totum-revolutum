use std::collections::BinaryHeap;

use crate::common::steps::syncronize::{
    PipelineStepSyncronizeEnd, PipelineStepSyncronizeStart, SyncronizeMarked, SyncronizeMarkedTrait,
};
use crate::{Message, PipelineData};
use flume::Sender;
use tracing::debug;

use crate::sync::steps::PipelineStepSync;

impl<Input: PipelineData> PipelineStepSync<Input, SyncronizeMarked<Input>> for PipelineStepSyncronizeStart {
    fn run<I: Iterator<Item = Input>>(&self, source: I, target: Sender<Message<SyncronizeMarked<Input>>>) {
        for (i, it) in source.into_iter().enumerate() {
            if let Err(e) = target.send(Message::Data(SyncronizeMarked::new(i, it))) {
                debug!("Error sending from blanket implementation of PipelineSyncStart: {e}");
            }
        }
    }
}

impl<InnerInput: PipelineData, Input: SyncronizeMarkedTrait<InnerInput>> PipelineStepSync<Input, InnerInput>
    for PipelineStepSyncronizeEnd
{
    fn run<I: Iterator<Item = Input>>(&self, source: I, target: Sender<Message<InnerInput>>) {
        let mut next = 0;
        let mut heap = BinaryHeap::new();
        for it in source {
            heap.push(it);

            while let Some(peek) = heap.peek() {
                if peek.mark() == next {
                    let item = heap.pop().expect("Already checked above");
                    if let Err(e) = target.send(Message::Data(item.into_inner())) {
                        debug!("Error sending from blanket implementation of PipelineSyncStart: {e}");
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
    use super::*;
    use crate::sync::steps::tests::collect_rx;

    #[test]
    fn test_syncronize_step_start() {
        let sync_start = PipelineStepSyncronizeStart;

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || sync_start.run(10..12, tx));

        let r = collect_rx(rx);
        assert_eq!(r, vec![SyncronizeMarked::new(0, 10), SyncronizeMarked::new(1, 11)]);
    }

    #[test]
    fn test_syncronize_step_end() {
        let sync_end = PipelineStepSyncronizeEnd;

        let (tx, rx) = flume::bounded(2);
        let data = vec![
            SyncronizeMarked::new(0, 10),
            SyncronizeMarked::new(2, 12),
            SyncronizeMarked::new(1, 11),
        ];
        std::thread::spawn(move || sync_end.run(data.into_iter(), tx));

        let r = collect_rx(rx);
        assert_eq!(r, vec![10, 11, 12]);
    }
}
