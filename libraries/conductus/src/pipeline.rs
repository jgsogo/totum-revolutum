#[cfg(feature = "tokio-async")]
use crate::r#async::PipelineTailOpsAsync;
#[cfg(feature = "sync")]
use crate::sync::PipelineTailSyncOps;
use crate::{PipelineData, PipelineHeadImpl, PipelineTailImpl};

/// The pipeline object. Start here.
pub struct Pipeline<Input: PipelineData, Output: PipelineData> {
    pub(crate) head: PipelineHeadImpl<Input>,
    pub(crate) tail: PipelineTailImpl<Output>,
}

impl<Input: PipelineData> Pipeline<Input, Input> {
    /// Creates a new [`Pipeline`] with an initial buffer with the given capacity `cap`.
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

impl<Input: PipelineData, Output: PipelineData> Pipeline<Input, Output> {
    /// Returns the [`PipelineHeadImpl`] and [`PipelineTailImpl`] of the [`Pipeline`]
    pub fn head_and_tail(self) -> (PipelineHeadImpl<Input>, PipelineTailImpl<Output>) {
        (self.head, self.tail)
    }

    /// Concat this [`Pipeline`] with another one
    #[cfg(feature = "sync")]
    pub fn concat_sync<NextOutput: PipelineData>(
        self,
        other: Pipeline<Output, NextOutput>,
    ) -> Pipeline<Input, NextOutput> {
        let (other_head, other_tail) = other.head_and_tail();
        self.tail.concat_sync(other_head);
        Pipeline {
            head: self.head,
            tail: other_tail,
        }
    }

    /// Concat this [`Pipeline`] with another one
    #[cfg(feature = "tokio-async")]
    pub async fn concat_async<NextOutput: PipelineData>(
        self,
        other: Pipeline<Output, NextOutput>,
    ) -> Pipeline<Input, NextOutput>
    where
        Output: Sync,
    {
        let (other_head, other_tail) = other.head_and_tail();
        self.tail.concat_async(other_head).await;
        Pipeline {
            head: self.head,
            tail: other_tail,
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
    fn test_unbounded() {
        let pipeline = Pipeline::empty(None);
        pipeline.send_sync(1).unwrap();
        pipeline.send_sync(2).unwrap();
        pipeline.send_sync(3).unwrap();
        pipeline.send_sync(4).unwrap();
        pipeline.send_sync(5).unwrap();

        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_head_and_tail() {
        let pipeline = Pipeline::empty(None);

        let (head, tail) = pipeline.head_and_tail();
        head.send_sync(1).unwrap();
        head.send_sync(2).unwrap();
        drop(head);

        let out = tail.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![1, 2]);
    }

    #[tokio::test]
    async fn test_concat() {
        let pipeline1 = Pipeline::empty(None);
        let pipeline2 = Pipeline::empty(None);
        let pipeline3 = Pipeline::empty(None);

        let p12 = pipeline1.concat_sync(pipeline2);
        let p123 = p12.concat_async(pipeline3).await;

        p123.send_async(1).await.unwrap();
        let out = p123.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![1]);
    }
}
