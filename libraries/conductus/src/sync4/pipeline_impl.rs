use std::fmt::Debug;
use std::marker::PhantomData;
use std::thread::JoinHandle;

use flume::SendError;

use crate::sync4::head::PipelineHead;
use crate::sync4::pipeline_ops::PipelineTailOpsFamily;
use crate::sync4::steps::PipelineStep;
use crate::sync4::{PipelineHeadImpl, PipelineTail, PipelineTailOps};

pub struct PipelineImpl<
    Input: Send + 'static,
    Output: Send + 'static + Debug,
    Head: PipelineHead<TInput = Input>,
    Tail: PipelineTailOps<Output>,
> {
    head: Head,
    tail: Tail,
    _output: PhantomData<Output>,
}

impl<Input: Send + 'static + Debug> PipelineImpl<Input, Input, PipelineHeadImpl<Input>, PipelineTail<Input>> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTail::new(rx),
            _output: PhantomData,
        }
    }
}

pub struct PipelineImplFamily<Output, Head, Tail> {
    // _input: PhantomData<Input>,
    _output: PhantomData<Output>,
    _head: PhantomData<Head>,
    _tail: PhantomData<Tail>,
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input> + 'static,
        Tail: PipelineTailOps<Output>,
    > PipelineTailOpsFamily for PipelineImplFamily<Output, Head, Tail>
{
    type PipelineTailOps<NextOutput: Send + 'static + Debug> =
        PipelineImpl<Input, NextOutput, Head, <Tail::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput>>;
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input> + 'static,
        Tail: PipelineTailOps<Output>,
    > PipelineTailOps<Output> for PipelineImpl<Input, Output, Head, Tail>
{
    type Family = PipelineImplFamily<Output, Head, Tail>;

    fn trait_pipe<NextOutput: Send + 'static + Debug, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineImpl<Input, NextOutput, Head, <Tail::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.trait_pipe(step, cap);
        PipelineImpl {
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
    ) -> PipelineImpl<Input, NextOutput, Head, <Tail::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput>>
    {
        let tail = self.tail.trait_parallel_pipe(step, workers, cap);
        PipelineImpl {
            head: self.head,
            tail,
            _output: PhantomData,
        }
    }
    //
    //     // fn trait_pipe<NextOutput: Send + 'static, PS: PipelineStep<Self::Output, NextOutput> + Send + 'static>(
    //     //     self,
    //     //     step: PS,
    //     //     cap: usize,
    //     // ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
    //     //     let tail = self.tail.trait_pipe(step, cap);
    //     //     PipelineImpl { head: self.head, tail }
    //     // }
    //
    //     // fn trait_parallel_pipe<
    //     //     NextOutput: Send + 'static,
    //     //     PS: PipelineStep<Self::Output, NextOutput> + Send + 'static + Copy,
    //     // >(
    //     //     self,
    //     //     step: PS,
    //     //     workers: usize,
    //     //     cap: usize,
    //     // ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
    //     //     let tail = self.tail.trait_parallel_pipe(step, workers, cap);
    //     //     PipelineImpl { head: self.head, tail }
    //     // }
}

impl<
        Input: Send + 'static,
        Output: Send + 'static + Debug,
        Head: PipelineHead<TInput = Input>,
        Tail: PipelineTailOps<Output>,
    > PipelineHead for PipelineImpl<Input, Output, Head, Tail>
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
