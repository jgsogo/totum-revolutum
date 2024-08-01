use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

/// The pipeline object. Start here.
pub struct Pipeline<Input: PipelineData, Output: PipelineData> {
    pub(crate) head: PipelineHeadImpl<Input>,
    pub(crate) tail: PipelineTailImpl<Output>,
}

impl<Input: PipelineData> Pipeline<Input, Input> {
    /// Creates a new [`Pipeline`] with a initial buffer with the given capacity `cap`.
    ///
    /// A capacity equal to `None` will use an unbounded channel, while `Some(cap)` will create a
    /// bounded channel with capacity `cap` (see [`flume::bounded`] documentation).
    ///
    /// # Examples
    ///
    /// Example for a sync world
    ///
    /// ```
    /// use conductus::Pipeline;
    /// use conductus::sync::PipelineHeadSync;
    /// use conductus::sync::PipelineTailSync;
    ///
    /// let pipeline = Pipeline::empty(None).map(|v: i32| v*2, None);
    ///
    /// pipeline.send_sync(2).unwrap();
    /// pipeline.send_sync(3).unwrap();
    ///
    /// let out = pipeline.into_iter().collect::<Vec<_>>();
    /// assert_eq!(out, vec![4, 6]);
    /// ```
    ///
    /// Async example:
    ///
    /// ```
    /// use conductus::Pipeline;
    /// use conductus::r#async::PipelineHeadAsync;
    /// use conductus::r#async::PipelineTailAsync;
    ///
    /// use futures::StreamExt;
    /// use futures::FutureExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///
    ///     let pipeline = Pipeline::empty(None)
    ///         .map(move |value| async move { value  *2 }, None)
    ///         .await;
    ///
    ///     pipeline.send_async(2).await.unwrap();
    ///     pipeline.send_async(3).await.unwrap();
    ///
    ///     let out = pipeline.into_stream().collect::<Vec<_>>().await;
    ///     assert_eq!(out, vec![4, 6]);
    /// }
    /// ```
    pub fn empty(cap: Option<usize>) -> Self {
        let (tx, rx) = match cap {
            None => flume::unbounded(),
            Some(cap) => flume::bounded(cap),
        };

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
    use crate::sync::{PipelineHeadSync, PipelineTailSync};
    use crate::Pipeline;

    #[tokio::test]
    async fn test_sync_async() {
        // Even though we expect people to use sync or async implementations of conductus, nothing
        // prevents them from using both.

        let step_sync = PipelineSyncStepMap::from(|value| value);
        let step_async = PipelineAsyncStepMap::from(move |value| async move { value });
        let pipeline = Pipeline::empty(Some(2))
            .pipe_sync(step_sync, Some(2))
            .pipe_async(step_async, Some(2))
            .await;

        pipeline.send_async(10).await.unwrap();
        pipeline.send_async(1).await.unwrap();

        let out = pipeline.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }

    #[test]
    fn test_unbouded() {
        let pipeline = Pipeline::empty(None);
        pipeline.send_sync(1).unwrap();
        pipeline.send_sync(2).unwrap();
        pipeline.send_sync(3).unwrap();
        pipeline.send_sync(4).unwrap();
        pipeline.send_sync(5).unwrap();

        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }
}
