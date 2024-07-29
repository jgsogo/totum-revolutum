use async_trait::async_trait;
use std::marker::PhantomData;

use flume::SendError;
use tokio::task::JoinHandle;

use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

use super::{PipelineHeadAsync, PipelineTailAsync};

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

#[async_trait]
impl<
        Input: PipelineData,
        Output: PipelineData + Sync,
        Head: PipelineHeadAsync<TInput = Input> + Sync,
        Tail: PipelineTailAsync<Output> + Sync,
    > PipelineHeadAsync for PipelineAsync<Input, Output, Head, Tail>
{
    type TInput = Input;

    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self._head.send(item).await
    }

    async fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>
    where
        <I as IntoIterator>::IntoIter: Send,
    {
        self._head.send_detached(input).await
    }
}
