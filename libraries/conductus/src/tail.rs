use crate::{Message, PipelineData};

/// A default implementation of a pipeline tail.
pub struct PipelineTailImpl<Output: PipelineData> {
    pub(crate) rx: flume::Receiver<Message<Output>>,
}

impl<Output: PipelineData> PipelineTailImpl<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }
}

impl<Output: Clone + PipelineData> PipelineTailImpl<Output> {
    pub fn split(self) -> (PipelineTailImpl<Output>, PipelineTailImpl<Output>) {
        let (tx_lhs, rx_lhs) = flume::unbounded();
        let (tx_rhs, rx_rhs) = flume::unbounded();

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
}
