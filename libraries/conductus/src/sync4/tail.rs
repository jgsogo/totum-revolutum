use std::fmt::Debug;

use crate::sync4::pipeline::Message;
use crate::sync4::pipeline_ops::PipelineTailOpsFamily;
use crate::sync4::steps::PipelineStep;
use crate::sync4::PipelineTailOps;

pub struct PipelineTail<Output: Debug> {
    rx: flume::Receiver<Message<Output>>,
}

impl<Output: Send + 'static + Debug> PipelineTail<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }
}

pub struct PipelineTailFamily;

impl PipelineTailOpsFamily for PipelineTailFamily {
    type PipelineTailOps<NextOutput: Send + 'static + Debug> = PipelineTail<NextOutput>;
}

impl<Output: Send + 'static + Debug> PipelineTailOps<Output> for PipelineTail<Output> {
    type Family = PipelineTailFamily;

    fn trait_pipe<NextOutput: Send + 'static + Debug, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineTail<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || step.run(self, tx));
        PipelineTail::new(rx)
    }

    fn trait_parallel_pipe<
        NextOutput: Send + 'static + Debug,
        PS: PipelineStep<Output, NextOutput> + Send + 'static + Copy,
    >(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> PipelineTail<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        for _ in 0..workers {
            let tx = tx.clone();
            let tail = PipelineTail::new(self.rx.clone());
            std::thread::spawn(move || step.run(tail, tx));
        }
        PipelineTail::new(rx)
    }
}

impl<Output: Clone + Send + 'static + Debug> PipelineTail<Output> {
    pub fn split(self) -> (PipelineTail<Output>, PipelineTail<Output>) {
        let (tx_lhs, rx_lhs) = flume::unbounded();
        let (tx_rhs, rx_rhs) = flume::unbounded();

        std::thread::spawn(move || {
            for it in self.rx {
                tx_lhs.send(it.clone()).unwrap();
                tx_rhs.send(it.clone()).unwrap();
            }
        });

        let tail_lhs = PipelineTail::new(rx_lhs);
        let tail_rhs = PipelineTail::new(rx_rhs);
        (tail_lhs, tail_rhs)
    }
}

impl<Output: Debug> IntoIterator for PipelineTail<Output> {
    type Item = Output;
    type IntoIter = PipelineTailIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        PipelineTailIter { tail: self }
    }
}

pub struct PipelineTailIter<Output: Debug> {
    tail: PipelineTail<Output>,
}

impl<Output: Debug> Iterator for PipelineTailIter<Output> {
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

    use crate::sync4::steps::{PipelineStepMap, SyncMarkedTrait};

    use super::*;

    #[test]
    fn test_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let tail = PipelineTail::new(rx).trait_pipe(step, 2);

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
        let tail = PipelineTail::new(rx).trait_parallel_pipe(step, 2, 2);

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
        let tail = PipelineTail::new(rx).trait_map(|value| value * 2, 2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![20, 2]);
    }

    #[test]
    fn test_parallel_map() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTail::new(rx).trait_parallel_map(
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
        let tail = PipelineTail::new(rx).trait_map(|value| value, 2).buffer(2);

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
        let tail = PipelineTail::new(rx).window(|value| value.into_iter().cloned().collect::<Vec<_>>(), 2, 4);

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
        let tail = PipelineTail::new(rx)
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
