use super::{PipelineHeadAsync, PipelineTailAsync};
use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};
use std::marker::PhantomData;

pub struct PipelineAsync<
    Input: PipelineData,
    Output: PipelineData,
    Head: PipelineHeadAsync<TInput = Input>,
    Tail: PipelineTailAsync<Output>,
> {
    _head: Head,
    _tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: PipelineData> PipelineAsync<Input, Input, PipelineHeadImpl<Input>, PipelineTailImpl<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            _head: PipelineHeadImpl::new(tx),
            _tail: PipelineTailImpl::new(rx),
            _output: PhantomData,
        }
    }
}
