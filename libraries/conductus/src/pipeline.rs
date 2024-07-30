use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

pub struct Pipeline<Input: PipelineData, Output: PipelineData> {
    pub(crate) head: PipelineHeadImpl<Input>,
    pub(crate) tail: PipelineTailImpl<Output>,
}

impl<Input: PipelineData> Pipeline<Input, Input> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHeadImpl::new(tx),
            tail: PipelineTailImpl::new(rx),
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use crate::r#async::steps::PipelineStepMap as PipelineAsyncStepMap;
    use crate::r#async::PipelineHeadAsync;
    use crate::r#async::PipelineTailAsync;
    use crate::sync::steps::PipelineStepMap as PipelineSyncStepMap;
    use crate::sync::PipelineTailSync;
    use crate::Pipeline;

    #[tokio::test]
    async fn test_sync_async() {
        // Even though we expect people to use sync or async implementations of conductus, nothing
        // prevents them from using both.

        let step_sync = PipelineSyncStepMap::from(|value| value);
        let step_async = PipelineAsyncStepMap::from(move |value| async move { value });
        let pipeline = Pipeline::empty(2)
            .pipe_sync(step_sync, 2)
            .pipe_async(step_async, 2)
            .await;

        pipeline.send_async(10).await.unwrap();
        pipeline.send_async(1).await.unwrap();

        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }
}
