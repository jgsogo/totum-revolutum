use crate::r#async::steps::PipelineStep;
use crate::sync::PipelineData;
use async_trait::async_trait;

pub trait PipelineTailFamily {
    type PipelineTailOps<NextOutput: PipelineData>: PipelineTail<NextOutput>;
}

/// Interface for the tail of a pipeline
#[async_trait]
pub trait PipelineTail<Output: PipelineData>: Sized {
    type Family: PipelineTailFamily;

    /// Adds a [`PipelineStep`] to the pipeline
    async fn pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput>>(
        self,
        step: PS,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;

    /// Adds a [`PipelineStep`] to the pipeline. This step will be executed in parallel using as
    /// many workers as given
    async fn parallel_pipe<NextOutput: PipelineData, PS: PipelineStep<Output, NextOutput> + Copy>(
        self,
        step: PS,
        workers: usize,
        cap: usize,
    ) -> <Self::Family as PipelineTailFamily>::PipelineTailOps<NextOutput>;
}
