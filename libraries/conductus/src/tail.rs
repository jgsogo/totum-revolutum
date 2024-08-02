use crate::{Message, PipelineData};

/// A default implementation of a pipeline tail.
pub struct PipelineTailImpl<Output: PipelineData> {
    pub(crate) rx: flume::Receiver<Message<Output>>,
}

impl<Output: PipelineData> PipelineTailImpl<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }
}

impl<Output: Clone + PipelineData> PipelineTailImpl<Output> {
    pub fn split(self) -> (PipelineTailImpl<Output>, PipelineTailImpl<Output>) {
        let (tx_lhs, rx_lhs) = flume::unbounded();
        let (tx_rhs, rx_rhs) = flume::unbounded();

        std::thread::spawn(move || {
            for it in self.rx {
                tx_lhs.send(it.clone()).unwrap();
                tx_rhs.send(it.clone()).unwrap();
            }
        });

        let tail_lhs = PipelineTailImpl::new(rx_lhs);
        let tail_rhs = PipelineTailImpl::new(rx_rhs);
        (tail_lhs, tail_rhs)
    }
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;

    use crate::r#async::steps::PipelineStepMap as PipelineAsyncStepMap;
    use crate::r#async::PipelineTailAsync;
    use crate::sync::steps::PipelineStepMap as PipelineSyncStepMap;
    use crate::sync::PipelineTailSync;
    use crate::{Message, PipelineTailImpl};

    #[tokio::test]
    async fn test_sync_async() {
        // Even though we expect people to use sync or async implementations of conductus, nothing
        // prevents them from using both.

        let (tx, rx) = flume::bounded(0);
        let step_sync = PipelineSyncStepMap::from(|value| value);
        let step_async = PipelineAsyncStepMap::from(move |value| async move { value });
        let tail = PipelineTailImpl::new(rx)
            .pipe_sync(step_sync, Some(2))
            .pipe_async(step_async, Some(2))
            .await;

        tx.send_async(Message::Data(10)).await.unwrap();
        tx.send_async(Message::Data(1)).await.unwrap();
        drop(tx);

        let out = tail.into_stream().collect::<Vec<_>>().await;
        assert_eq!(out, vec![10, 1]);
    }
}
