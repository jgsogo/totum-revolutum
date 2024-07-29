use std::marker::PhantomData;
use std::thread::JoinHandle;

use crate::{PipelineData, PipelineTailImpl};
use flume::SendError;

use crate::sync::steps::PipelineStepSync;
use crate::sync::tail::PipelineTailSyncFamily;
use crate::sync::{PipelineHeadSync, PipelineTailSync};
use crate::tail::PipelineTailImplIter;
use crate::PipelineHeadImpl;

/// Implementation of a pipeline. This object acts both as a [`PipelineHeadSync`] and a
/// [`PipelineTailSync`].
pub struct PipelineSync<
    Input: PipelineData,
    Output: PipelineData,
    Head: PipelineHeadSync<TInput = Input>,
    Tail: PipelineTailSync<Output>,
> {
    head: Head,
    tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: PipelineData> PipelineSync<Input, Input, PipelineHeadImpl<Input>, PipelineTailImpl<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTailImpl::new(rx),
            _output: PhantomData,
        }
    }
}

impl<Input: PipelineData, Output: PipelineData, Head: PipelineHeadSync<TInput = Input>>
    PipelineSync<Input, Output, Head, PipelineTailImpl<Output>>
{
    pub fn drain(self) -> PipelineTailImplIter<Output> {
        self.tail.into_iter()
    }
}

/// A helper struct to implement [`PipelineTailSyncFamily`], so that [`PipelineSync`] can implement the
/// [`PipelineTailSync`] trait.
pub struct PipelineImplFamily<Output, Head, Tail> {
    _output: PhantomData<Output>,
    _head: PhantomData<Head>,
    _tail: PhantomData<Tail>,
}

impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadSync<TInput = Input> + 'static,
        Tail: PipelineTailSync<Output>,
    > PipelineTailSyncFamily for PipelineImplFamily<Output, Head, Tail>
{
    type PipelineTailOps<NextOutput: PipelineData> =
        PipelineSync<Input, NextOutput, Head, <Tail::Family as PipelineTailSyncFamily>::PipelineTailOps<NextOutput>>;
}

impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadSync<TInput = Input> + 'static,
        Tail: PipelineTailSync<Output>,
    > PipelineTailSync<Output> for PipelineSync<Input, Output, Head, Tail>
{
    type Family = PipelineImplFamily<Output, Head, Tail>;

    fn pipe<NextOutput: PipelineData, PS: PipelineStepSync<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineSync<Input, NextOutput, Head, <Tail::Family as PipelineTailSyncFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.pipe(step, cap);
        PipelineSync {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }

    fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStepSync<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> PipelineSync<Input, NextOutput, Head, <Tail::Family as PipelineTailSyncFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.parallel_pipe(step, workers, cap);
        PipelineSync {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }
}

impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadSync<TInput = Input>,
        Tail: PipelineTailSync<Output>,
    > PipelineHeadSync for PipelineSync<Input, Output, Head, Tail>
{
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

impl<
        Input: PipelineData,
        Output: PipelineData,
        Head: PipelineHeadSync<TInput = Input>,
        Tail: PipelineTailSync<Output> + IntoIterator<Item = Output>,
    > IntoIterator for PipelineSync<Input, Output, Head, Tail>
{
    type Item = Output;
    type IntoIter = Tail::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
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
        let pipeline = PipelineSync::empty(2).pipe(step, 2);

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
        let pipeline = PipelineSync::empty(2).parallel_pipe(step, 2, 2);

        pipeline.send(10).unwrap();
        pipeline.send(1).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 10]);
    }

    #[test]
    fn test_send() {
        let pipeline = PipelineSync::empty(2);

        pipeline.send(3).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![3]);
    }

    #[test]
    fn test_send_detached() {
        let pipeline = PipelineSync::empty(2);

        pipeline.send_detached(0..3);
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2]);
    }

    #[test]
    fn test_drain() {
        let pipeline = PipelineSync::empty(2);
        pipeline.send_detached(0..5);

        let out = pipeline.drain().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2, 3, 4]);
    }
}
