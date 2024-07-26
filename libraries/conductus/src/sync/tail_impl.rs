use crate::sync::steps::PipelineStep;
use crate::sync::tail::PipelineTailFamily;
use crate::sync::PipelineTail;
use crate::sync::{Message, PipelineData};

pub struct PipelineTailImpl<Output: PipelineData> {
    rx: flume::Receiver<Message<Output>>,
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

impl<Output: PipelineData> PipelineTail<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailImplFamily;

    fn trait_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineTailImpl<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || step.run(self, tx));
        PipelineTailImpl::new(rx)
    }

    fn trait_parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> PipelineTailImpl<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        for _ in 0..workers {
            let tx = tx.clone();
            let tail = PipelineTailImpl::new(self.rx.clone());
            std::thread::spawn(move || step.run(tail, tx));
        }
        PipelineTailImpl::new(rx)
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
                Message::Flush => None,
            },
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::sync::steps::{PipelineStepMap, SyncMarkedTrait};

    use super::*;

    #[test]
    fn test_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let tail = PipelineTailImpl::new(rx).trait_pipe(step, 2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![10, 1]);
    }

    #[test]
    fn test_parallel_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let tail = PipelineTailImpl::new(rx).trait_parallel_pipe(step, 2, 2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        tx.send(Message::Data(2)).unwrap();
        tx.send(Message::Data(3)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 2, 3, 10]);
    }

    #[test]
    fn test_map() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx).trait_map(|value| value * 2, 2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![20, 2]);
    }

    #[test]
    fn test_parallel_map() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx).trait_parallel_map(
            |value| {
                std::thread::sleep(Duration::from_millis(value * 10u64));
                value * 2
            },
            2,
            2,
        );

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        tx.send(Message::Data(2)).unwrap();
        tx.send(Message::Data(3)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![2, 4, 6, 20]);
    }

    #[test]
    fn test_buffer() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx).trait_map(|value| value, 2).buffer(2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        tx.send(Message::Data(2)).unwrap();
        tx.send(Message::Data(3)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![10, 1, 2, 3]);
    }

    #[test]
    fn test_window() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx).window(|value| value.into_iter().cloned().collect::<Vec<_>>(), 2, 4);

        tx.send(Message::Data(0)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        tx.send(Message::Data(2)).unwrap();
        tx.send(Message::Data(3)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![vec![0, 1], vec![1, 2], vec![2, 3]]);
    }

    #[test]
    fn test_sync() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx)
            .sync_mark()
            .trait_parallel_map(
                |value| {
                    let inner = value.inner();
                    std::thread::sleep(Duration::from_millis((inner * 10) as u64));
                    value
                },
                2,
                2,
            )
            .sync();

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        tx.send(Message::Data(2)).unwrap();
        tx.send(Message::Data(3)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![10, 1, 2, 3]);
    }
}
