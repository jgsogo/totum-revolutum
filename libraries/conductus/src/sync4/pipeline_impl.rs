use crate::sync4::steps::PipelineStep;
use crate::sync4::{PipelineHead, PipelineTailOps};

pub struct PipelineImpl<Input, Output: Send + 'static, Tail: PipelineTailOps<Output = Output>> {
    head: PipelineHead<Input>,
    tail: Tail,
}

impl<Input: 'static, Output: Send + 'static, Tail: PipelineTailOps<Output = Output>> PipelineTailOps
    for PipelineImpl<Input, Output, Tail>
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
