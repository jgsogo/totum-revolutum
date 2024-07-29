use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Debug;

use crate::{Message, PipelineData};
use flume::Sender;
use tracing::debug;

use crate::sync::steps::PipelineStep;

/// A [`PipelineStep`] that wraps every input into a [`SyncMarked`]. This wrapper contains a
/// mark that can be used by [`PipelineStepSyncEnd`] to reorder the stream of data to
/// match the input order.
pub struct PipelineStepSyncStart;

pub trait SyncMarkedTrait<Input: PipelineData>: Ord + PipelineData {
    fn into_inner(self) -> Input;

    fn inner(&self) -> &Input;
    fn mark(&self) -> usize;
}

#[derive(Debug)]
pub struct SyncMarked<Input: PipelineData> {
    mark: usize,
    value: Input,
}

impl<Input: PipelineData> SyncMarkedTrait<Input> for SyncMarked<Input> {
    fn into_inner(self) -> Input {
        self.value
    }

    fn inner(&self) -> &Input {
        &self.value
    }

    fn mark(&self) -> usize {
        self.mark
    }
}

impl<Input: PipelineData> SyncMarked<Input> {
    pub fn new(i: usize, value: Input) -> Self {
        Self { mark: i, value }
    }
}

impl<Input: PipelineData> Eq for SyncMarked<Input> {}

impl<Input: PipelineData> PartialEq<Self> for SyncMarked<Input> {
    fn eq(&self, other: &Self) -> bool {
        self.mark == other.mark
    }
}

impl<Input: PipelineData> PartialOrd<Self> for SyncMarked<Input> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Input: PipelineData> Ord for SyncMarked<Input> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mark.cmp(&self.mark)
    }
}

impl<Input: PipelineData> PipelineStep<Input, SyncMarked<Input>> for PipelineStepSyncStart {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<SyncMarked<Input>>>) {
        for (i, it) in source.into_iter().enumerate() {
            if let Err(e) = target.send(Message::Data(SyncMarked::new(i, it))) {
                debug!("Error sending from blanket implementation of PipelineSyncStart: {e}");
            }
        }
    }
}

/// A [`PipelineStep`] that can be added to a pipeline to reorder a stream of [`SyncMarked`] data
/// following the input order.
pub struct PipelineStepSyncEnd;

impl<InnerInput: PipelineData, Input: SyncMarkedTrait<InnerInput>> PipelineStep<Input, InnerInput>
    for PipelineStepSyncEnd
{
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<InnerInput>>) {
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

    #[test]
    fn test_sync_step_start() {
        let sync_start = PipelineStepSyncStart;

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || sync_start.run(10..12, tx));

        let r = rx
            .into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![SyncMarked::new(0, 10), SyncMarked::new(1, 11)]);
    }

    #[test]
    fn test_sync_step_end() {
        let sync_end = PipelineStepSyncEnd;

        let (tx, rx) = flume::bounded(2);
        let data = vec![SyncMarked::new(0, 10), SyncMarked::new(2, 12), SyncMarked::new(1, 11)];
        std::thread::spawn(move || sync_end.run(data.into_iter(), tx));

        let r = rx
            .into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![10, 11, 12]);
    }
}
