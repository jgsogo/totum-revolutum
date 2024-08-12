use std::cmp::Ordering;

use crate::PipelineData;

/// A pipeline step that wraps every input into a [`SynchronizeMarked`]. This wrapper contains a
/// mark that can be used by [`PipelineStepSynchronizeEnd`] to reorder the stream of data to
/// match the input order.
pub struct PipelineStepSynchronizeStart;

/// A pipeline step that can be added to a pipeline to reorder a stream of [`SynchronizeMarked`] data
/// following the input order.
pub struct PipelineStepSynchronizeEnd;

pub trait SynchronizeMarkedTrait<Input: PipelineData>: Ord + PipelineData {
    fn into_inner(self) -> Input;

    fn inner(&self) -> &Input;
    fn mark(&self) -> usize;
}

impl<Input: PipelineData> SynchronizeMarkedTrait<Input> for SynchronizeMarked<Input> {
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
pub struct SynchronizeMarked<Input: PipelineData> {
    mark: usize,
    value: Input,
}

impl<Input: PipelineData> SynchronizeMarked<Input> {
    pub fn new(i: usize, value: Input) -> Self {
        Self { mark: i, value }
    }
}

impl<Input: PipelineData> Eq for SynchronizeMarked<Input> {}

impl<Input: PipelineData> PartialEq<Self> for SynchronizeMarked<Input> {
    fn eq(&self, other: &Self) -> bool {
        self.mark == other.mark
    }
}

impl<Input: PipelineData> PartialOrd<Self> for SynchronizeMarked<Input> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Input: PipelineData> Ord for SynchronizeMarked<Input> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mark.cmp(&self.mark)
    }
}
