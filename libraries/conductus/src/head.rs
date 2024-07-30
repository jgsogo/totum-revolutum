use crate::{Message, PipelineData};

/// A default implementation of a pipeline head
pub struct PipelineHeadImpl<Input: PipelineData> {
    pub(crate) tx: flume::Sender<Message<Input>>,
}

impl<Input: PipelineData> PipelineHeadImpl<Input> {
    pub(crate) fn new(tx: flume::Sender<Message<Input>>) -> Self {
        Self { tx }
    }
}

#[cfg(test)]
mod tests {
    use crate::r#async::PipelineHeadAsync;
    use crate::sync::PipelineHeadSync;
    use crate::{Message, PipelineHeadImpl};

    #[tokio::test]
    async fn test_sync_async() {
        // Even though we expect people to use sync or async implementations of conductus, nothing
        // prevents them from using both.

        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        PipelineHeadSync::send_sync(&head, 10).unwrap();
        PipelineHeadAsync::send_async(&head, 42).await.unwrap();
        drop(head);

        let received = rx.into_iter().collect::<Vec<_>>();
        assert_eq!(received, vec![Message::Data(10), Message::Data(42),])
    }
}
