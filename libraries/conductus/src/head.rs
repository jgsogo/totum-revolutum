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
