use super::{PipelineHead, PipelineHeadImpl, PipelineTail, PipelineTailImpl};
use crate::sync::PipelineData;
use std::marker::PhantomData;

pub struct Pipeline<
    Input: PipelineData,
    Output: PipelineData,
    Head: PipelineHead<TInput = Input>,
    Tail: PipelineTail<Output>,
> {
    _head: Head,
    _tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: PipelineData> Pipeline<Input, Input, PipelineHeadImpl<Input>, PipelineTailImpl<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            _head: PipelineHeadImpl::new(tx),
            _tail: PipelineTailImpl::new(rx),
            _output: PhantomData,
        }
    }
}
