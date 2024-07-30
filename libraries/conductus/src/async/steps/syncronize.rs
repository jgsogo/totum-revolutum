/// A [`PipelineStepASync`] that wraps every input into a [`SyncronizeMarked`]. This wrapper contains a
/// mark that can be used by [`PipelineStepSyncronizeEnd`] to reorder the stream of data to
/// match the input order.
pub struct PipelineStepSyncronizeStart;

pub struct PipelineStepSyncronizeEnd;
