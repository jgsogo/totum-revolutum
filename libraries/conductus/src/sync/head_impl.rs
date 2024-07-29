use crate::sync::PipelineHead;
use crate::{Message, PipelineData};
use flume::SendError;
use std::thread::JoinHandle;

/// A default implementation of a pipeline head
pub struct PipelineHeadImpl<Input: PipelineData> {
    tx: flume::Sender<Message<Input>>,
}

impl<Input: PipelineData> PipelineHeadImpl<Input> {
    pub(crate) fn new(tx: flume::Sender<Message<Input>>) -> Self {
        Self { tx }
    }
}

impl<Input: PipelineData> PipelineHead for PipelineHeadImpl<Input> {
    type TInput = Input;

    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.tx.send(Message::Data(item)).map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>> {
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let r: Result<Vec<_>, _> = input
                .into_iter()
                .map(|it| {
                    tx.send(Message::Data(it)).map_err(|e| {
                        let Message::Data(data) = e.into_inner() else {
                            unreachable!()
                        };
                        SendError(data)
                    })
                })
                .collect();
            r.map(|_| ())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::head::PipelineHead;

    #[test]
    fn test_send() {
        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        head.send(10).unwrap();
        head.send(42).unwrap();
        drop(head);

        let received = rx.into_iter().collect::<Vec<_>>();
        assert_eq!(received, vec![Message::Data(10), Message::Data(42),])
    }

    #[test]
    fn test_send_batch() {
        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        head.send_batch(1..3);
        drop(head);

        let received = rx.into_iter().collect::<Vec<_>>();
        assert_eq!(received, vec![Message::Data(1), Message::Data(2),])
    }
}
