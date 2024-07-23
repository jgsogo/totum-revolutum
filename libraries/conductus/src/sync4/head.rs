use crate::sync4::pipeline::Message;
use flume::SendError;
use std::thread::JoinHandle;

pub trait PipelineHead {
    type Input: Send + 'static;

    fn send(&self, item: Self::Input) -> Result<(), SendError<Self::Input>>;

    fn send_batch<I: IntoIterator<Item = Self::Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::Input>>>;
}
pub struct PipelineHeadImpl<Input> {
    tx: flume::Sender<Message<Input>>,
}

impl<Input: Send + 'static> PipelineHeadImpl<Input> {
    pub(crate) fn new(tx: flume::Sender<Message<Input>>) -> Self {
        Self { tx }
    }
}

impl<Input: Send + 'static> PipelineHead for PipelineHeadImpl<Input> {
    type Input = Input;

    fn send(&self, item: Self::Input) -> Result<(), SendError<Self::Input>> {
        self.tx.send(Message::Data(item)).map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    fn send_batch<I: IntoIterator<Item = Self::Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Self::Input>>> {
        // TODO: We can send a BatchEnd message, maybe
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
