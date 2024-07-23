use std::thread::JoinHandle;

use flume::SendError;

use crate::sync4::head::PipelineHead;
use crate::sync4::steps::PipelineStep;
use crate::sync4::PipelineTailOps;

pub struct PipelineImpl<
    Input: Send + 'static,
    Output: Send + 'static,
    Head: PipelineHead<Input = Input>,
    Tail: PipelineTailOps<Output = Output>,
> {
    head: Head,
    tail: Tail,
}

impl<
        Input: Send + 'static,
        Output: Send + 'static,
        Head: PipelineHead<Input = Input> + 'static,
        Tail: PipelineTailOps<Output = Output>,
    > PipelineTailOps for PipelineImpl<Input, Output, Head, Tail>
{
    type Output = Output;

    fn trait_pipe<NextOutput: Send + 'static, PS: PipelineStep<Self::Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
        let tail = self.tail.trait_pipe(step, cap);
        PipelineImpl { head: self.head, tail }
    }

    fn trait_parallel_pipe<
        NextOutput: Send + 'static,
        PS: PipelineStep<Self::Output, NextOutput> + Send + 'static + Copy,
    >(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
        let tail = self.tail.trait_parallel_pipe(step, workers, cap);
        PipelineImpl { head: self.head, tail }
    }
}

impl<
        Input: Send + 'static,
        Output: Send + 'static,
        Head: PipelineHead<Input = Input>,
        Tail: PipelineTailOps<Output = Output>,
    > PipelineHead for PipelineImpl<Input, Output, Head, Tail>
{
    type Input = Input;

    fn send(&self, item: Self::Input) -> Result<(), SendError<Self::Input>> {
        self.head.send(item)
    }

    fn send_batch<I: IntoIterator<Item = Self::Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::Input>>> {
        self.head.send_batch(input)
    }
}
