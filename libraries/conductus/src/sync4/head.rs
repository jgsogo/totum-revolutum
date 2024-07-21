use crate::sync4::pipeline::Message;
use flume::SendError;
use std::thread::JoinHandle;

pub struct PipelineHead<Input> {
    tx: flume::Sender<Message<Input>>,
}

impl<Input: Send + 'static> PipelineHead<Input> {
    pub(crate) fn new(tx: flume::Sender<Message<Input>>) -> Self {
        Self { tx }
    }

    pub fn send(&self, item: Input) -> Result<(), SendError<Input>> {
        self.tx.send(Message::Data(item)).map_err(|e| {
            let Message::Data(msg) = e.into_inner() else {
                unreachable!()
            };
            SendError(msg)
        })
    }

    pub fn send_batch<I: IntoIterator<Item = Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Input>>> {
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
