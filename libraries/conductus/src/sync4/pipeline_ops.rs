use std::collections::VecDeque;

use crate::sync4::steps::{
    PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepSyncEnd, PipelineStepSyncStart, PipelineStepWindow,
    SyncMarked, SyncMarkedTrait,
};
pub trait PipelineTailOpsFamily {
    type PipelineTailOps<NextOutput: Send + 'static>: PipelineTailOps<NextOutput>;
}

pub trait PipelineTailOps<Output: Send + 'static>: Sized {
    type Family: PipelineTailOpsFamily;

    fn trait_pipe<NextOutput: Send + 'static, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput>;

    fn trait_parallel_pipe<NextOutput: Send + 'static, PS: PipelineStep<Output, NextOutput> + Send + 'static + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput>;

    fn trait_map<NextOutput: Send + 'static, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        cap: usize,
    ) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_pipe(step, cap)
    }

    fn trait_parallel_map<NextOutput: Send + 'static, Func: Fn(Output) -> NextOutput + Send + 'static + Copy>(
        self,
        func: Func,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_parallel_pipe(step, workers, cap)
    }

    fn buffer(self, cap: usize) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<Output> {
        self.trait_pipe(PipelineStepNoop, cap)
    }

    fn window<NextOutput: Send + 'static, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<NextOutput> {
        let step = PipelineStepWindow::new(func, window_size);
        self.trait_pipe(step, cap)
    }

    fn sync_mark(self) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<SyncMarked<Output>> {
        self.trait_pipe(PipelineStepSyncStart, 0)
    }

    fn sync<InnerOutput: Send + 'static>(self) -> <Self::Family as PipelineTailOpsFamily>::PipelineTailOps<InnerOutput>
    where
        Output: SyncMarkedTrait<InnerOutput>,
    {
        self.trait_pipe(PipelineStepSyncEnd, 0)
    }
}
