use crate::sync::PipelineTailFamily;
use crate::{Message, PipelineData};
use tracing::debug;

pub struct PipelineTailImpl<Output: PipelineData> {
    pub(crate) rx: flume::Receiver<Message<Output>>,
}

impl<Output: PipelineData> PipelineTailImpl<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }
}

/// A helper struct to implement [`PipelineTailFamily`], so that [`PipelineTailImpl`] can implement
/// the [`PipelineTail`] trait.
pub struct PipelineTailImplFamily;

impl PipelineTailFamily for PipelineTailImplFamily {
    type PipelineTailOps<NextOutput: PipelineData> = PipelineTailImpl<NextOutput>;
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

impl<Output: PipelineData> IntoIterator for PipelineTailImpl<Output> {
    type Item = Output;
    type IntoIter = PipelineTailIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        PipelineTailIter { tail: self }
    }
}

pub struct PipelineTailIter<Output: PipelineData> {
    tail: PipelineTailImpl<Output>,
}

impl<Output: PipelineData> Iterator for PipelineTailIter<Output> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tail.rx.recv() {
            Ok(msg) => match msg {
                Message::Data(data) => Some(data),
                Message::Stop(reason) => {
                    debug!("Stop iteration due to data error: {reason}");
                    None
                }
            },
            Err(e) => {
                debug!("Stop iteration due to receive error: {e}");
                None
            }
        }
    }
}
