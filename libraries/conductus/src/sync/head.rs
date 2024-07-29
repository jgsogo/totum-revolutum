use std::thread::JoinHandle;

use flume::SendError;

use crate::{Message, PipelineData, PipelineHeadImpl};

/// Interface for everything that can act as the head of a pipeline
pub trait PipelineHeadSync {
    type TInput: PipelineData;

    /// Sends one item into the head of the pipeline. This is a blocking call.
    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>>;

    fn send_batch<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> Result<(), SendError<Self::TInput>> {
        for it in input {
            self.send(it)?;
        }
        Ok(())
    }

    /// Sends several items into the head of the pipeline.
    ///
    /// Implementors should detach the caller from the actual send into the channels.
    fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::TInput>>>;
}

impl<Input: PipelineData> PipelineHeadSync for PipelineHeadImpl<Input> {
    type TInput = Input;

    fn send(&self, item: Self::TInput) -> Result<(), SendError<Self::TInput>> {
        self.tx.send(Message::Data(item)).map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    fn send_detached<I: IntoIterator<Item = Self::TInput> + Send + 'static>(
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

        head.send_batch(0..3).unwrap();
        drop(head);

        let received = rx.into_iter().collect::<Vec<_>>();
        assert_eq!(received, vec![Message::Data(0), Message::Data(1), Message::Data(2),])
    }

    #[test]
    fn test_send_detached() {
        let (tx, rx) = flume::bounded(20);
        let head = PipelineHeadImpl::new(tx);

        head.send_detached(1..3);
        drop(head);

        let received = rx.into_iter().collect::<Vec<_>>();
        assert_eq!(received, vec![Message::Data(1), Message::Data(2),])
    }
}
