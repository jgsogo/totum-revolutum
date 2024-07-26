use crate::sync::PipelineData;
use std::collections::VecDeque;

use crate::sync::steps::{
    PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepSyncEnd, PipelineStepSyncStart, PipelineStepWindow,
    SyncMarked, SyncMarkedTrait,
};
pub trait PipelineTailFamily {
    type PipelineTailOps<NextOutput: PipelineData>: PipelineTail<NextOutput>;
}

pub trait PipelineTail<Output: PipelineData>: Sized {
    type Family: PipelineTailFamily;

    fn trait_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    fn trait_parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Send + 'static + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    fn trait_map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_pipe(step, cap)
    }

    fn trait_parallel_map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static + Copy>(
        self,
        func: Func,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_parallel_pipe(step, workers, cap)
    }

    fn buffer(self, cap: usize) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<Output> {
        self.trait_pipe(PipelineStepNoop, cap)
    }

    fn window<NextOutput: PipelineData, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step = PipelineStepWindow::new(func, window_size);
        self.trait_pipe(step, cap)
    }

    fn sync_mark(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<SyncMarked<Output>> {
        self.trait_pipe(PipelineStepSyncStart, 0)
    }

    fn sync<InnerOutput: PipelineData>(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<InnerOutput>
    where
        Output: SyncMarkedTrait<InnerOutput>,
    {
        self.trait_pipe(PipelineStepSyncEnd, 0)
    }
}
