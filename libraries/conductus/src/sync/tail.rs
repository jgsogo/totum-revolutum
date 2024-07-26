use crate::sync::PipelineData;
use std::collections::VecDeque;

use crate::sync::steps::{
    PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepSyncEnd, PipelineStepSyncStart, PipelineStepWindow,
    SyncMarked, SyncMarkedTrait,
};
pub trait PipelineTailFamily {
    type PipelineTailOps<NextOutput: PipelineData>: PipelineTail<NextOutput>;
}

/// Interface for the tail of a pipeline
pub trait PipelineTail<Output: PipelineData>: Sized {
    type Family: PipelineTailFamily;

    /// Adds a [`PipelineStep`] to the pipeline
    fn trait_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStep`] to the pipeline. This step will be executed in parallel using as
    /// many workers as given
    fn trait_parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStepMap`] with the function given
    fn trait_map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_pipe(step, cap)
    }

    /// Adds a parallel [`PipelineStepMap`] using the function given with as many workers as given
    /// in the argument.
    fn trait_parallel_map<NextOutput: PipelineData, Func: Fn(Output) -> NextOutput + Send + 'static + Copy>(
        self,
        func: Func,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = func.into();
        self.trait_parallel_pipe(step, workers, cap)
    }

    /// Adds a [`PipelineStepNoop`] step with the given buffer.
    fn buffer(self, cap: usize) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<Output> {
        self.trait_pipe(PipelineStepNoop, cap)
    }

    /// Adds a [`PipelineStepWindow`] executing the function given as argument
    fn window<NextOutput: PipelineData, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput> {
        let step = PipelineStepWindow::new(func, window_size);
        self.trait_pipe(step, cap)
    }

    /// Adds a [`PipelineStepSyncStart`] step to the tail of the pipeline
    fn sync_mark(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<SyncMarked<Output>> {
        self.trait_pipe(PipelineStepSyncStart, 0)
    }

    /// Adds a [`PipelineStepSyncEnd`] step to the tail of the pipeline
    fn sync<InnerOutput: PipelineData>(self) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<InnerOutput>
    where
        Output: SyncMarkedTrait<InnerOutput>,
    {
        self.trait_pipe(PipelineStepSyncEnd, 0)
    }
}
