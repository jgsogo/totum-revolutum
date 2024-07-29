use crate::{Message, PipelineData};
use flume::r#async::RecvStream;
use futures::stream::FusedStream;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use tracing::debug;

pub struct PipelineTailImpl<Output: PipelineData> {
    pub(crate) rx: flume::Receiver<Message<Output>>,
}

impl<Output: PipelineData> PipelineTailImpl<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }

    pub fn stream(&self) -> PipelineTailImplStream<Output> {
        PipelineTailImplStream(self.rx.stream())
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

impl<Output: PipelineData> IntoIterator for PipelineTailImpl<Output> {
    type Item = Output;
    type IntoIter = PipelineTailImplIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        PipelineTailImplIter { tail: self }
    }
}

pub struct PipelineTailImplIter<Output: PipelineData> {
    tail: PipelineTailImpl<Output>,
}

impl<Output: PipelineData> Iterator for PipelineTailImplIter<Output> {
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
