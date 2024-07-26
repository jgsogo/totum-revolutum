use std::fmt::Debug;
use std::marker::PhantomData;
use std::thread::JoinHandle;

use flume::SendError;

use crate::sync::steps::PipelineStep;
use crate::sync::tail::PipelineTailFamily;
use crate::sync::tail_impl::PipelineTailIter;
use crate::sync::{PipelineHead, PipelineHeadImpl, PipelineTail, PipelineTailImpl};

pub struct Pipeline<
    Input: Send + 'static,
    Output: Send + 'static + Debug,
    Head: PipelineHead<TInput = Input>,
    Tail: PipelineTail<Output>,
> {
    head: Head,
    tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: Send + 'static + Debug> Pipeline<Input, Input, PipelineHeadImpl<Input>, PipelineTailImpl<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTailImpl::new(rx),
            _output: PhantomData,
        }
    }
}

impl<Input: Send + 'static, Output: Send + 'static + Debug, Head: PipelineHead<TInput = Input>>
    Pipeline<Input, Output, Head, PipelineTailImpl<Output>>
{
    pub fn drain(self) -> PipelineTailIter<Output> {
        self.tail.into_iter()
    }
}

pub struct PipelineImplFamily<Output, Head, Tail> {
    _output: PhantomData<Output>,
    _head: PhantomData<Head>,
    _tail: PhantomData<Tail>,
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input> + 'static,
        Tail: PipelineTail<Output>,
    > PipelineTailFamily for PipelineImplFamily<Output, Head, Tail>
{
    type PipelineTailOps<NextOutput: Send + 'static + Debug> =
        Pipeline<Input, NextOutput, Head, <Tail::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>>;
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input> + 'static,
        Tail: PipelineTail<Output>,
    > PipelineTail<Output> for Pipeline<Input, Output, Head, Tail>
{
    type Family = PipelineImplFamily<Output, Head, Tail>;

    fn trait_pipe<NextOutput: Send + 'static + Debug, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> Pipeline<Input, NextOutput, Head, <Tail::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>> {
        let tail = self.tail.trait_pipe(step, cap);
        Pipeline {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }

    fn trait_parallel_pipe<
        NextOutput: Send + 'static + Debug,
        PS: PipelineStep<Output, NextOutput> + Send + 'static + Copy,
    >(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> Pipeline<Input, NextOutput, Head, <Tail::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>> {
        let tail = self.tail.trait_parallel_pipe(step, workers, cap);
        Pipeline {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input>,
        Tail: PipelineTail<Output>,
    > PipelineHead for Pipeline<Input, Output, Head, Tail>
{
    type TInput = Input;

    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.head.send(item)
    }

    fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        self.head.send_batch(input)
    }
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input>,
        Tail: PipelineTail<Output> + IntoIterator<Item = Output>,
    > IntoIterator for Pipeline<Input, Output, Head, Tail>
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
        let pipeline = Pipeline::empty(2).trait_pipe(step, 2);

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
        let pipeline = Pipeline::empty(2).trait_parallel_pipe(step, 2, 2);

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
    fn test_send_batch() {
        let pipeline = Pipeline::empty(2);

        pipeline.send_batch(0..3);
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2]);
    }

    #[test]
    fn test_drain() {
        let pipeline = Pipeline::empty(2);
        pipeline.send_batch(0..5);

        let out = pipeline.drain().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2, 3, 4]);
    }
}
