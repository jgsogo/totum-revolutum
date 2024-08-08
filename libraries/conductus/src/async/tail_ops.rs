use async_trait::async_trait;
use futures::StreamExt;

use crate::r#async::tail::PipelineTailAsyncImplFamily;
use crate::r#async::{PipelineHeadAsync, PipelineTailAsync, PipelineTailAsyncFamily};
use crate::{Message, PipelineData, PipelineTailImpl};

#[async_trait]
pub trait PipelineTailOpsAsync<Output: PipelineData + Sync>: Sized {
    type Family: PipelineTailAsyncFamily;

    async fn split(
        self,
        cap: Option<usize>,
    ) -> (
        <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
        <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
    )
    where
        Output: Clone;

    async fn merge(
        self,
        other: <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
        cap: Option<usize>,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>;

    async fn concat<Head: PipelineHeadAsync<TInput = Output> + Send>(self, head: Head);
}

#[async_trait]
impl<Output: PipelineData + Sync> PipelineTailOpsAsync<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailAsyncImplFamily;

    async fn split(
        self,
        cap: Option<usize>,
    ) -> (
        <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
        <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
    )
    where
        Output: Clone,
    {
        let (tx_lhs, rx_lhs) = match cap {
            None => flume::unbounded(),
            Some(cap) => flume::bounded(cap),
        };
        let (tx_rhs, rx_rhs) = match cap {
            None => flume::unbounded(),
            Some(cap) => flume::bounded(cap),
        };

        tokio::spawn(async move {
            let mut stream = self.rx.into_stream();
            while let Some(it) = stream.next().await {
                let _ = tx_lhs.send_async(it.clone()).await;
                let _ = tx_rhs.send_async(it).await;
            }
        });

        (PipelineTailImpl::new(rx_lhs), PipelineTailImpl::new(rx_rhs))
    }

    async fn merge(
        self,
        other: <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output>,
        cap: Option<usize>,
    ) -> <Self::Family as PipelineTailAsyncFamily>::PipelineTailOps<Output> {
        let (tx, rx) = match cap {
            None => flume::unbounded(),
            Some(cap) => flume::bounded(cap),
        };

        let tx_cloned = tx.clone();
        tokio::spawn(async move {
            let mut stream = self.rx.into_stream();
            while let Some(it) = stream.next().await {
                let _ = tx_cloned.send_async(it).await;
            }
        });

        tokio::spawn(async move {
            let mut stream = other.into_stream();
            while let Some(it) = stream.next().await {
                let _ = tx.send_async(Message::Data(it)).await;
            }
        });

        PipelineTailImpl::new(rx)
    }

    async fn concat<Head: PipelineHeadAsync<TInput = Output> + Send>(self, head: Head) {
        head.send_detached(self.into_stream()).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#async::steps::tests::collect_rx;
    use crate::PipelineHeadImpl;

    #[tokio::test]
    async fn test_split() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx);

        let (tail1, tail2) = tail.split(Some(2)).await;

        tx.send_async(Message::Data(10)).await.unwrap();
        tx.send_async(Message::Data(1)).await.unwrap();
        drop(tx);

        let out1 = tail1.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out1, vec![10, 1]);
        let out2 = tail2.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out2, vec![10, 1]);
    }

    #[tokio::test]
    async fn test_merge() {
        let (tx, rx) = flume::bounded(2);
        let tail1 = PipelineTailImpl::new(rx);

        tx.send_async(Message::Data(1)).await.unwrap();
        tx.send_async(Message::Data(2)).await.unwrap();
        drop(tx);

        let (tx, rx) = flume::bounded(2);
        let tail2 = PipelineTailImpl::new(rx);

        tx.send_async(Message::Data(3)).await.unwrap();
        tx.send_async(Message::Data(4)).await.unwrap();
        drop(tx);

        let tail = tail1.merge(tail2, None).await;
        let mut out = collect_rx(tail.rx).await;
        out.sort();
        assert_eq!(out, vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_concat() {
        let (tx, tail) = {
            let (tx, rx) = flume::bounded(2);
            let tail = PipelineTailImpl::new(rx);
            (tx, tail)
        };

        let (rx, head) = {
            let (tx, rx) = flume::bounded(2);
            let head = PipelineHeadImpl::new(tx);
            (rx, head)
        };

        tail.concat(head).await;

        tx.send_async(Message::Data(3)).await.unwrap();
        tx.send_async(Message::Data(4)).await.unwrap();
        drop(tx);

        let out = collect_rx(rx).await;
        assert_eq!(out, vec![3, 4]);
    }
}
