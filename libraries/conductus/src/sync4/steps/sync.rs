use std::cmp::Ordering;
use std::collections::BinaryHeap;

use flume::Sender;
use tracing::debug;

use crate::sync4::pipeline::Message;
use crate::sync4::steps::PipelineStep;

pub struct PipelineStepSyncStart;

pub trait SyncMarkedTrait<Input>: Ord {
    fn into_inner(self) -> Input;

    fn inner(&self) -> &Input;
    fn mark(&self) -> usize;
}
pub struct SyncMarked<Input> {
    mark: usize,
    value: Input,
}

impl<Input> SyncMarkedTrait<Input> for SyncMarked<Input> {
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

impl<Input> SyncMarked<Input> {
    pub fn new(i: usize, value: Input) -> Self {
        Self { mark: i, value }
    }
}

impl<Input> Eq for SyncMarked<Input> {}

impl<Input> PartialEq<Self> for SyncMarked<Input> {
    fn eq(&self, other: &Self) -> bool {
        self.mark == other.mark
    }
}

impl<Input> PartialOrd<Self> for SyncMarked<Input> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Input> Ord for SyncMarked<Input> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mark.cmp(&self.mark)
    }
}

impl<Input> PipelineStep<Input, SyncMarked<Input>> for PipelineStepSyncStart {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<SyncMarked<Input>>>) {
        for (i, it) in source.into_iter().enumerate() {
            if let Err(e) = target.send(Message::Data(SyncMarked::new(i, it))) {
                debug!("Error sending from blanket implementation of PipelineSyncStart: {e}");
            }
        }
    }
}

pub struct PipelineStepSyncEnd;
//
// impl<Input> PipelineStep<SyncMarked<Input>, Input> for PipelineStepSyncEnd {
//     fn run<I: IntoIterator<Item = SyncMarked<Input>>>(&self, source: I, target: Sender<Message<Input>>) {
//         let mut next = 0;
//         let mut heap = BinaryHeap::new();
//         for it in source {
//             heap.push(it);
//
//             while let Some(peek) = heap.peek() {
//                 if peek.mark == next {
//                     let item = heap.pop().expect("Already checked above");
//                     if let Err(e) = target.send(Message::Data(item.value)) {
//                         debug!("Error sending from blanket implementation of PipelineSyncStart: {e}");
//                     }
//                     next += 1;
//                 } else {
//                     assert!(peek.mark > next);
//                     break;
//                 }
//             }
//         }
//     }
// }

impl<InnerInput, Input: SyncMarkedTrait<InnerInput>> PipelineStep<Input, InnerInput> for PipelineStepSyncEnd {
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
