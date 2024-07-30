use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

pub struct Pipeline<Input: PipelineData, Output: PipelineData> {
    pub(crate) head: PipelineHeadImpl<Input>,
    pub(crate) tail: PipelineTailImpl<Output>,
}

impl<Input: PipelineData> Pipeline<Input, Input> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTailImpl::new(rx),
        }
    }
}
