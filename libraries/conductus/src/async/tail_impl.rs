use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{Message, PipelineData};
use async_trait::async_trait;
use flume::r#async::RecvStream;
use futures::stream::FusedStream;
use futures::Stream;
use tracing::debug;

use crate::r#async::steps::PipelineStep;
use crate::r#async::tail::{PipelineTail, PipelineTailFamily};

pub struct PipelineTailImpl<Output: PipelineData> {
    rx: flume::Receiver<Message<Output>>,
}

impl<Output: PipelineData> PipelineTailImpl<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }

    pub fn stream(&self) -> PipelineTailImplStream<Output> {
        PipelineTailImplStream(self.rx.stream())
    }
}

/// A helper struct to implement [`PipelineTailFamily`], so that [`PipelineTailImpl`] can implement
/// the [`PipelineTail`] trait.
pub struct PipelineTailImplFamily;

impl PipelineTailFamily for PipelineTailImplFamily {
    type PipelineTailOps<NextOutput: PipelineData> = PipelineTailImpl<NextOutput>;
}

#[async_trait]
impl<Output: PipelineData> PipelineTail<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailImplFamily;

    async fn pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        tokio::spawn(async move {
            step.run(self.stream(), tx).await;
        });
        PipelineTailImpl::new(rx)
    }

    async fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        for _ in 0..workers {
            let tx = tx.clone();
            let tail = PipelineTailImpl::new(self.rx.clone());
            tokio::spawn(async move {
                step.run(tail.stream(), tx).await;
            });
        }
        PipelineTailImpl::new(rx)
    }
}

pub struct PipelineTailImplStream<'a, Output: PipelineData>(RecvStream<'a, Message<Output>>);

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

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

        let out = tail.stream().collect::<Vec<_>>().await;
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

        let out = tail.stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![1, 2, 3, 10]);
    }
}
