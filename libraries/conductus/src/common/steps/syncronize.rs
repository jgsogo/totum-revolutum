use std::cmp::Ordering;

use crate::PipelineData;

/// A pipeline step that wraps every input into a [`SyncronizeMarked`]. This wrapper contains a
/// mark that can be used by [`PipelineStepSyncronizeEnd`] to reorder the stream of data to
/// match the input order.
pub struct PipelineStepSyncronizeStart;

/// A pipeline step that can be added to a pipeline to reorder a stream of [`SyncronizeMarked`] data
/// following the input order.
pub struct PipelineStepSyncronizeEnd;

pub trait SyncronizeMarkedTrait<Input: PipelineData>: Ord + PipelineData {
    fn into_inner(self) -> Input;

    fn inner(&self) -> &Input;
    fn mark(&self) -> usize;
}

impl<Input: PipelineData> SyncronizeMarkedTrait<Input> for SyncronizeMarked<Input> {
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

#[derive(Debug)]
pub struct SyncronizeMarked<Input: PipelineData> {
    mark: usize,
    value: Input,
}

impl<Input: PipelineData> SyncronizeMarked<Input> {
    pub fn new(i: usize, value: Input) -> Self {
        Self { mark: i, value }
    }
}

impl<Input: PipelineData> Eq for SyncronizeMarked<Input> {}

impl<Input: PipelineData> PartialEq<Self> for SyncronizeMarked<Input> {
    fn eq(&self, other: &Self) -> bool {
        self.mark == other.mark
    }
}

impl<Input: PipelineData> PartialOrd<Self> for SyncronizeMarked<Input> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Input: PipelineData> Ord for SyncronizeMarked<Input> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mark.cmp(&self.mark)
    }
}
