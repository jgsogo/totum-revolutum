use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use flume::r#async::RecvStream;
use futures::stream::FusedStream;
use futures::Stream;
use tracing::debug;

use crate::r#async::steps::PipelineStepAsync;
use crate::r#async::steps::PipelineStepMap;
use crate::{Message, PipelineData, PipelineTailImpl};

pub trait PipelineTailAsyncFamily {
    type PipelineTailOps<NextOutput: PipelineData + Sync>: PipelineTailAsync<NextOutput> + Send;
}

/// A helper struct to implement [`PipelineTailAsyncFamily`], so that [`PipelineTailImpl`] can implement
/// the [`PipelineTailAsync`] trait.
pub struct PipelineTailAsyncImplFamily;

impl PipelineTailAsyncFamily for PipelineTailAsyncImplFamily {
    type PipelineTailOps<NextOutput: PipelineData + Sync> = PipelineTailImpl<NextOutput>;
}

/// Interface for the tail of a pipeline
#[async_trait]
pub trait PipelineTailAsync<Output: PipelineData + Sync>: Sized {
    type Family: PipelineTailAsyncFamily;

    /// Adds a [`PipelineStepAsync`] to the pipeline
    async fn pipe_async<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>;

    /// An alias to [`PipelineTailAsync::pipe_async`]
    async fn pipe<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput> {
        self.pipe_async(step, cap).await
    }

    /// Adds a [`PipelineStepAsync`] to the pipeline. This step will be executed in parallel using as
    /// many workers as given
    async fn parallel_pipe<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>;

    fn into_stream(self) -> PipelineTailImplStream<'static, Output>;

    /// Adds a [`PipelineStepMap`] with the function given
    async fn map<NextOutput: PipelineData + Sync, Fut, F>(
        self,
        func: F,
        cap: usize,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<NextOutput>
    where
        F: Fn(Output) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = NextOutput> + Send + 'static,
    {
        let step: PipelineStepMap<Output, NextOutput, Fut, F> = func.into();
        self.pipe_async(step, cap).await
    }
}

pub struct PipelineTailImplStream<'a, Output: PipelineData>(RecvStream<'a, Message<Output>>);

impl<'a, Output: PipelineData> PipelineTailImplStream<'a, Output> {
    pub fn new(recv: RecvStream<'a, Message<Output>>) -> Self {
        Self(recv)
    }
}

impl<Output: PipelineData> Stream for PipelineTailImplStream<'_, Output> {
    type Item = Output;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(m) => match m {
                None => Poll::Ready(None),
                Some(v) => match v {
                    Message::Data(d) => Poll::Ready(Some(d)),
                    Message::Stop(reason) => {
                        debug!("Stop iteration due to data error: {reason}");
                        Poll::Ready(None)
                    }
                },
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a, Output: PipelineData> FusedStream for PipelineTailImplStream<'a, Output> {
    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }
}

#[async_trait]
impl<Output: PipelineData + Sync> PipelineTailAsync<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailAsyncImplFamily;

    async fn pipe_async<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput>>(
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

    async fn parallel_pipe<NextOutput: PipelineData + Sync, PS: PipelineStepAsync<Output, NextOutput> + Copy>(
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

    use futures::StreamExt;

    use crate::common::steps::noop::PipelineStepNoop;
    use crate::Message;

    use super::*;

    #[tokio::test]
    async fn test_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepNoop;
        let tail = PipelineTailImpl::new(rx).pipe_async(step, 2).await;

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

    #[tokio::test]
    async fn test_map() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx)
            .map(move |value: u64| async move { value * 2 }, 2)
            .await;

        tx.send_async(Message::Data(10)).await.unwrap();
        tx.send_async(Message::Data(1)).await.unwrap();
        drop(tx);

        let out = tail.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![20, 2]);
    }
}
