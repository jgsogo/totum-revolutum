use std::collections::VecDeque;

use crate::sync4::steps::{
    PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepSyncEnd, PipelineStepSyncStart, PipelineStepWindow,
    SyncMarked, SyncMarkedTrait,
};

pub trait PipelineTailOps: Sized {
    type Output: Send + 'static;

    fn trait_pipe<NextOutput: Send + 'static, PS: PipelineStep<Self::Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static;

    fn trait_parallel_pipe<
        NextOutput: Send + 'static,
        PS: PipelineStep<Self::Output, NextOutput> + Send + 'static + Copy,
    >(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static;

    fn trait_map<NextOutput: Send + 'static, Func: Fn(Self::Output) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
        let step: PipelineStepMap<Self::Output, NextOutput, Func> = func.into();
        self.trait_pipe(step, cap)
    }

    fn trait_parallel_map<NextOutput: Send + 'static, Func: Fn(Self::Output) -> NextOutput + Send + 'static + Copy>(
        self,
        func: Func,
        workers: usize,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
        let step: PipelineStepMap<Self::Output, NextOutput, Func> = func.into();
        self.trait_parallel_pipe(step, workers, cap)
    }

    fn buffer(self, cap: usize) -> impl PipelineTailOps<Output = Self::Output> + 'static {
        self.trait_pipe(PipelineStepNoop, cap)
    }

    fn window<NextOutput: Send + 'static, Func: Fn(&VecDeque<Self::Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> impl PipelineTailOps<Output = NextOutput> + 'static {
        let step = PipelineStepWindow::new(func, window_size);
        self.trait_pipe(step, cap)
    }

    fn sync_mark(self) -> impl PipelineTailOps<Output = SyncMarked<Self::Output>> + 'static {
        self.trait_pipe(PipelineStepSyncStart, 0)
    }

    fn sync<InnerOutput: Send + 'static>(self) -> impl PipelineTailOps<Output = InnerOutput> + 'static
    where
        Self::Output: SyncMarkedTrait<InnerOutput>,
    {
        self.trait_pipe(PipelineStepSyncEnd, 0)
    }
}
