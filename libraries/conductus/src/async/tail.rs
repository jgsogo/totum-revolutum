use crate::r#async::steps::PipelineStepAsync;
use crate::tail::PipelineTailImplStream;
use crate::{PipelineData, PipelineTailImpl};
use async_trait::async_trait;

pub trait PipelineTailAsyncFamily {
    type PipelineTailOps<NextOutput: PipelineData>: PipelineTailAsync<NextOutput> + Send;
}

/// A helper struct to implement [`PipelineTailAsyncFamily`], so that [`PipelineTailImpl`] can implement
/// the [`PipelineTailAsync`] trait.
pub struct PipelineTailAsyncImplFamily;

impl PipelineTailAsyncFamily for PipelineTailAsyncImplFamily {
    type PipelineTailOps<NextOutput: PipelineData> = PipelineTailImpl<NextOutput>;
}

/// Interface for the tail of a pipeline
#[async_trait]
pub trait PipelineTailAsync<Output: PipelineData>: Sized {
    type Family: PipelineTailAsyncFamily;

    /// Adds a [`PipelineStepAsync`] to the pipeline
    async fn pipe<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStepAsync`] to the pipeline. This step will be executed in parallel using as
    /// many workers as given
    async fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>;

    fn into_stream(self) -> PipelineTailImplStream<'static, Output>;
}

#[async_trait]
impl<Output: PipelineData> PipelineTailAsync<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailAsyncImplFamily;

    async fn pipe<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        tokio::spawn(async move {
            step.run(self.into_stream(), tx).await;
        });
        PipelineTailImpl::new(rx)
    }

    async fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        for _ in 0..workers {
            let tx = tx.clone();
            let tail = PipelineTailImpl::new(self.rx.clone());
            tokio::spawn(async move {
                step.run(tail.into_stream(), tx).await;
            });
        }
        PipelineTailImpl::new(rx)
    }

    fn into_stream(self) -> PipelineTailImplStream<'static, Output> {
        PipelineTailImplStream::new(self.rx.into_stream())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Message;
    use futures::StreamExt;

    use crate::r#async::steps::PipelineStepMap;
    use crate::r#async::steps::PipelineStepNoop;

    use super::*;

    #[tokio::test]
    async fn test_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepNoop;
        let tail = PipelineTailImpl::new(rx).pipe(step, 2).await;

        tx.send_async(Message::Data(10)).await.unwrap();
        tx.send_async(Message::Data(1)).await.unwrap();
        drop(tx);

        let out = tail.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }

    #[tokio::test]
    async fn test_parallel_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepMap::from(move |value: u64| async move {
            tokio::time::sleep(Duration::from_millis(value * 10u64)).await;
            value
        });
        let tail = PipelineTailImpl::new(rx).parallel_pipe(step, 2, 2).await;

        tx.send_async(Message::Data(10)).await.unwrap();
        tx.send_async(Message::Data(1)).await.unwrap();
        tx.send_async(Message::Data(2)).await.unwrap();
        tx.send_async(Message::Data(3)).await.unwrap();
        drop(tx);

        let out = tail.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![1, 2, 3, 10]);
    }
}
