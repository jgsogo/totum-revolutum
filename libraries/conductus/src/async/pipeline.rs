use std::marker::PhantomData;

use async_trait::async_trait;
use flume::SendError;
use futures::Stream;
use tokio::task::JoinHandle;

use crate::r#async::steps::PipelineStepAsync;
use crate::r#async::PipelineHeadAsync;
use crate::r#async::{PipelineTailAsync, PipelineTailAsyncFamily, PipelineTailImplStream};
use crate::{Pipeline, PipelineData};

#[async_trait]
impl<Input: PipelineData, Output: PipelineData> PipelineHeadAsync for Pipeline<Input, Output> {
    type TInput = Input;

    async fn send_async(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.head.send_async(item).await
    }

    async fn send_detached<I: Stream<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        self.head.send_detached(input).await
    }
}

/// A helper struct to implement [`PipelineTailAsyncFamily`], so that [`Pipeline`] can implement
/// the [`PipelineTailAsync`] trait.
pub struct PipelineAsyncFamily<Input: PipelineData> {
    _input: PhantomData<Input>,
}

impl<Input: PipelineData> PipelineTailAsyncFamily for PipelineAsyncFamily<Input> {
    type PipelineTailOps<NextOutput: PipelineData + Sync> = Pipeline<Input, NextOutput>;
}

#[async_trait]
impl<Input: PipelineData, Output: PipelineData + Sync> PipelineTailAsync<Output> for Pipeline<Input, Output> {
    type Family = PipelineAsyncFamily<Input>;

    async fn pipe_async<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput> {
        let tail = self.tail.pipe_async(step, cap).await;
        Pipeline { head: self.head, tail }
    }

    async fn parallel_pipe<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput> {
        let tail = self.tail.parallel_pipe(step, workers, cap).await;
        Pipeline { head: self.head, tail }
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
        let pipeline = Pipeline::empty(2).pipe_async(step, 2).await;

        pipeline.send_async(10).await.unwrap();
        pipeline.send_async(1).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }

    #[tokio::test]
    async fn test_parallel_pipe() {
        let step = PipelineStepMap::from(move |value| async move {
            tokio::time::sleep(Duration::from_millis(value * 10u64)).await;
            value
        });
        let pipeline = Pipeline::empty(2).parallel_pipe(step, 2, 2).await;

        pipeline.send_async(10).await.unwrap();
        pipeline.send_async(1).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![1, 10]);
    }

    #[tokio::test]
    async fn test_send() {
        let pipeline = Pipeline::empty(2);
        pipeline.send_async(0).await.unwrap();
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![0]);
    }

    #[tokio::test]
    async fn test_send_detached() {
        let pipeline = Pipeline::empty(2);
        pipeline.send_detached(stream::iter(0..5)).await;
        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![0, 1, 2, 3, 4]);
    }
}
