use async_trait::async_trait;
use flume::SendError;

use crate::r#async::head::PipelineHead;
use crate::sync::{Message, PipelineData};

pub struct PipelineHeadImpl<Input: PipelineData> {
    tx: flume::Sender<Message<Input>>,
}

#[async_trait]
impl<Input: PipelineData> PipelineHead for PipelineHeadImpl<Input> {
    type TInput = Input;

    async fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.tx.send_async(Message::Data(item)).await.map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    async fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> tokio::task::JoinHandle<Result<(), SendError<Input>>>
    where
        <I as IntoIterator>::IntoIter: Send,
    {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            for it in input {
                tx.send_async(Message::Data(it)).await.map_err(|e| {
                    let Message::Data(msg) = e.into_inner() else {
                        unreachable!()
                    };
                    SendError(msg)
                })?
            }
            Ok(())
        })
    }
}
