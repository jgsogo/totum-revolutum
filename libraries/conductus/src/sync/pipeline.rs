use std::marker::PhantomData;
use std::thread::JoinHandle;

use flume::SendError;

use crate::sync::steps::PipelineStepSync;
use crate::sync::tail::PipelineTailImplIter;
use crate::sync::{PipelineHeadSync, PipelineTailSync, PipelineTailSyncFamily};
use crate::{Pipeline, PipelineData};

impl<Input: PipelineData, Output: PipelineData> PipelineHeadSync for Pipeline<Input, Output> {
    type TInput = Input;

    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.head.send(item)
    }

    fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        self.head.send_detached(input)
    }
}

/// A helper struct to implement [`PipelineTailSyncFamily`], so that [`Pipeline`] can implement
/// the [`PipelineTailSync`] trait.
pub struct PipelineSyncFamily<Input: PipelineData> {
    _input: PhantomData<Input>,
}

impl<Input: PipelineData> PipelineTailSyncFamily for PipelineSyncFamily<Input> {
    type PipelineTailOps<NextOutput: PipelineData> = Pipeline<Input, NextOutput>;
}

impl<Input: PipelineData, Output: PipelineData> PipelineTailSync<Output> for Pipeline<Input, Output> {
    type Family = PipelineSyncFamily<Input>;

    fn pipe_sync<NextOutput: PipelineData, PS: PipelineStepSync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<NextOutput> {
        let tail = self.tail.pipe_sync(step, cap);
        Pipeline { head: self.head, tail }
    }

    fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStepSync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailSyncFamily>::PipelineTailOps<NextOutput> {
        let tail = self.tail.parallel_pipe(step, workers, cap);
        Pipeline { head: self.head, tail }
    }

    fn into_iter(self) -> PipelineTailImplIter<Output> {
        self.tail.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::sync::steps::PipelineStepMap;

    use super::*;

    #[test]
    fn test_pipe() {
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let pipeline = Pipeline::empty(2).pipe_sync(step, 2);

        pipeline.send(10).unwrap();
        pipeline.send(1).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![10, 1]);
    }

    #[test]
    fn test_parallel_pipe() {
        let step = PipelineStepMap::from(|value| {
            std::thread::sleep(Duration::from_millis(value * 10u64));
            value
        });
        let pipeline = Pipeline::empty(2).parallel_pipe(step, 2, 2);

        pipeline.send(10).unwrap();
        pipeline.send(1).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 10]);
    }

    #[test]
    fn test_send() {
        let pipeline = Pipeline::empty(2);

        pipeline.send(3).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![3]);
    }

    #[test]
    fn test_send_detached() {
        let pipeline = Pipeline::empty(2);

        pipeline.send_detached(0..3);
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2]);
    }
}
