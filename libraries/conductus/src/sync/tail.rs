use crate::{PipelineData, PipelineTailImpl};
use std::collections::VecDeque;

use crate::sync::steps::{
    PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepSyncEnd, PipelineStepSyncStart, PipelineStepWindow,
    SyncMarked, SyncMarkedTrait,
};
use crate::tail::PipelineTailImplFamily;

pub trait PipelineTailFamily {
    type PipelineTailOps<NextOutput: PipelineData>: PipelineTail<NextOutput>;
}

/// Interface for the tail of a pipeline
pub trait PipelineTail<Output: PipelineData>: Sized {
    type Family: PipelineTailFamily;

    /// Adds a [`PipelineStep`] to the pipeline
    fn pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStep`] to the pipeline. This step will be executed in parallel using as
    /// many workers as given
    fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStepMap`] with the function given
    fn map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.pipe(step, cap)
    }

    /// Adds a parallel [`PipelineStepMap`] using the function given with as many workers as given
    /// in the argument.
    fn parallel_map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static + Copy>(
        self,
        func: Func,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.parallel_pipe(step, workers, cap)
    }

    /// Adds a [`PipelineStepNoop`] step with the given buffer.
    fn buffer(self, cap: usize) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<Output> {
        self.pipe(PipelineStepNoop, cap)
    }

    /// Adds a [`PipelineStepWindow`] executing the function given as argument
    fn window<NextOutput: PipelineData, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step = PipelineStepWindow::new(func, window_size);
        self.pipe(step, cap)
    }

    /// Adds a [`PipelineStepSyncStart`] step to the tail of the pipeline
    fn sync_mark(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<SyncMarked<Output>> {
        self.pipe(PipelineStepSyncStart, 0)
    }

    /// Adds a [`PipelineStepSyncEnd`] step to the tail of the pipeline
    fn sync<InnerOutput: PipelineData>(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<InnerOutput>
    where
        Output: SyncMarkedTrait<InnerOutput>,
    {
        self.pipe(PipelineStepSyncEnd, 0)
    }
}

impl<Output: PipelineData> PipelineTail<Output> for PipelineTailImpl<Output> {
    type Family = PipelineTailImplFamily;

    fn pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineTailImpl<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || step.run(self, tx));
        PipelineTailImpl::new(rx)
    }

    fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
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

#[cfg(test)]
mod tests {
    use crate::sync::steps::{PipelineStepMap, SyncMarkedTrait};
    use crate::Message;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_pipe() {
        let (tx, rx) = flume::bounded(0);
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let tail = PipelineTailImpl::new(rx).pipe(step, 2);

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
        let tail = PipelineTailImpl::new(rx).parallel_pipe(step, 2, 2);

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
        let tail = PipelineTailImpl::new(rx).map(|value| value * 2, 2);

        tx.send(Message::Data(10)).unwrap();
        tx.send(Message::Data(1)).unwrap();
        drop(tx);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![20, 2]);
    }

    #[test]
    fn test_parallel_map() {
        let (tx, rx) = flume::bounded(0);
        let tail = PipelineTailImpl::new(rx).parallel_map(
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
        let tail = PipelineTailImpl::new(rx).map(|value| value, 2).buffer(2);

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
            .parallel_map(
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
