use crate::sync::tail::PipelineTailSyncImplFamily;
use crate::sync::{PipelineTailSync, PipelineTailSyncFamily};
use crate::{Message, PipelineData, PipelineTailImpl};

pub trait PipelineTailSyncOps<Output: PipelineData>: Sized {
    type Family: PipelineTailSyncFamily;

    fn split(
        self,
        cap: Option<usize>,
    ) -> (
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
    )
    where
        Output: Clone;

    fn merge(
        self,
        other: <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>,
        cap: Option<usize>,
    ) -> <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<Output>;
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
                tx_lhs.send(it.clone()).unwrap();
                tx_rhs.send(it.clone()).unwrap();
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
}
