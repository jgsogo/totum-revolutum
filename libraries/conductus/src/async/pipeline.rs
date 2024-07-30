use std::marker::PhantomData;

use async_trait::async_trait;
use flume::SendError;
use futures::Stream;
use tokio::task::JoinHandle;

use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

use super::steps::PipelineStepAsync;
use super::{PipelineHeadAsync, PipelineTailAsync, PipelineTailAsyncFamily, PipelineTailImplStream};

pub struct PipelineAsync<
    Input: PipelineData,
    Output: PipelineData,
    Head: PipelineHeadAsync<TInput = Input>,
    Tail: PipelineTailAsync<Output>,
> {
    head: Head,
    tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: PipelineData> PipelineAsync<Input, Input, PipelineHeadImpl<Input>, PipelineTailImpl<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTailImpl::new(rx),
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
        self.head.send(item).await
    }

    async fn send_detached<I: Stream<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        self.head.send_detached(input).await
    }
}

/// A helper struct to implement [`PipelineTailSyncFamily`], so that [`PipelineSync`] can implement the
/// [`PipelineTailSync`] trait.
pub struct PipelineAsyncFamily<Output, Head, Tail> {
    _output: PhantomData<Output>,
    _head: PhantomData<Head>,
    _tail: PhantomData<Tail>,
}

impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadAsync<TInput = Input> + Send,
        Tail: PipelineTailAsync<Output>,
    > PipelineTailAsyncFamily for PipelineAsyncFamily<Output, Head, Tail>
{
    type PipelineTailOps<NextOutput: PipelineData> =
        PipelineAsync<Input, NextOutput, Head, <Tail::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>>;
}

#[async_trait]
impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadAsync<TInput = Input> + Send,
        Tail: PipelineTailAsync<Output> + Send,
    > PipelineTailAsync<Output> for PipelineAsync<Input, Output, Head, Tail>
{
    type Family = PipelineAsyncFamily<Output, Head, Tail>;

    async fn pipe_async<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineAsync<Input, NextOutput, Head, <Tail::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.pipe_async(step, cap).await;
        PipelineAsync {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }

    async fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> PipelineAsync<Input, NextOutput, Head, <Tail::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.parallel_pipe(step, workers, cap).await;
        PipelineAsync {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }

    fn into_stream(self) -> PipelineTailImplStream<'static, Output> {
        self.tail.into_stream()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::super::steps::PipelineStepMap;
    use super::*;
    use futures::{stream, StreamExt};

    #[tokio::test]
    async fn test_pipe() {
        let step = PipelineStepMap::from(move |value| async move {
            tokio::time::sleep(Duration::from_millis(value * 10u64)).await;
            value
        });
        let pipeline = PipelineAsync::empty(2).pipe_async(step, 2).await;

        pipeline.send(10).await.unwrap();
        pipeline.send(1).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }

    #[tokio::test]
    async fn test_parallel_pipe() {
        let step = PipelineStepMap::from(move |value| async move {
            tokio::time::sleep(Duration::from_millis(value * 10u64)).await;
            value
        });
        let pipeline = PipelineAsync::empty(2).parallel_pipe(step, 2, 2).await;

        pipeline.send(10).await.unwrap();
        pipeline.send(1).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![1, 10]);
    }

    #[tokio::test]
    async fn test_send() {
        let pipeline = PipelineAsync::empty(2);
        pipeline.send(0).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![0]);
    }

    #[tokio::test]
    async fn test_send_detached() {
        let pipeline = PipelineAsync::empty(2);
        pipeline.send_detached(stream::iter(0..5)).await;
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![0, 1, 2, 3, 4]);
    }
}
