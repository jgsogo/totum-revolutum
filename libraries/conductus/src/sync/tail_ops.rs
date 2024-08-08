use crate::sync::tail::PipelineTailSyncImplFamily;
use crate::sync::{PipelineHeadSync, PipelineTailSync, PipelineTailSyncFamily};
use crate::{Message, PipelineData, PipelineTailImpl};

pub trait PipelineTailSyncOps<Output: PipelineData>: Sized {
    type Family: PipelineTailSyncFamily;

    /// Duplicates the output at this point and sends it to two different tail implementations
    fn split(
        self,
        cap: Option<usize>,
    ) -> (
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
    )
    where
        Output: Clone;

    /// Will combine the output from current tail and the `other` one and send them to a single
    /// tail (no order is guaranteed)
    fn merge(
        self,
        other: <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        cap: Option<usize>,
    ) -> <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>;

    /// Sends all the outputs from this tail to the given [`PipelineHeadSync`] implementation. This
    /// method consumes both objects as now these two pipeline endpoints are bounded together.
    fn concat_sync<Head: PipelineHeadSync<TInput = Output>>(self, head: Head);
}

impl<Output: PipelineData> PipelineTailSyncOps<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailSyncImplFamily;

    fn split(
        self,
        cap: Option<usize>,
    ) -> (
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
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

        std::thread::spawn(move || {
            for it in self.rx {
                let _ = tx_lhs.send(it.clone());
                let _ = tx_rhs.send(it);
            }
        });

        let tail_lhs = PipelineTailImpl::new(rx_lhs);
        let tail_rhs = PipelineTailImpl::new(rx_rhs);
        (tail_lhs, tail_rhs)
    }

    fn merge(
        self,
        other: <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        cap: Option<usize>,
    ) -> <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output> {
        let (tx, rx) = match cap {
            None => flume::unbounded(),
            Some(cap) => flume::bounded(cap),
        };

        let tx_lhs = tx.clone();
        std::thread::spawn(move || {
            for it in self.rx {
                tx_lhs.send(it).unwrap();
            }
        });
        std::thread::spawn(move || {
            for it in other.into_iter() {
                tx.send(Message::Data(it)).unwrap();
            }
        });
        PipelineTailImpl::new(rx)
    }

    fn concat_sync<Head: PipelineHeadSync<TInput = Output>>(self, head: Head) {
        head.send_detached(self.into_iter());
    }
}

#[cfg(test)]
mod tests {
    use crate::sync::steps::tests::collect_rx;
    use crate::PipelineHeadImpl;

    use super::*;

    #[test]
    fn test_split() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx);

        let (tail1, tail2) = tail.split(Some(2));

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        drop(tx);

        let out1 = tail1.into_iter().collect::<Vec<_>>();
        assert_eq!(out1, vec![10, 1]);
        let out2 = tail2.into_iter().collect::<Vec<_>>();
        assert_eq!(out2, vec![10, 1]);
    }

    #[test]
    fn test_merge() {
        let (tx1, rx1) = flume::bounded(2);
        let tail1 = PipelineTailImpl::new(rx1);

        let (tx2, rx2) = flume::bounded(2);
        let tail2 = PipelineTailImpl::new(rx2);

        tx1.send(Message::Data(1)).unwrap();
        tx1.send(Message::Data(2)).unwrap();
        drop(tx1);

        tx2.send(Message::Data(3)).unwrap();
        tx2.send(Message::Data(4)).unwrap();
        drop(tx2);

        let tail = tail1.merge(tail2, Some(2));

        let mut out = tail.into_iter().collect::<Vec<_>>();
        out.sort();
        assert_eq!(out, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_concat() {
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

        tail.concat_sync(head);

        tx.send(Message::Data(3)).unwrap();
        tx.send(Message::Data(4)).unwrap();
        drop(tx);

        let out = collect_rx(rx);
        assert_eq!(out, vec![3, 4]);
    }
}
