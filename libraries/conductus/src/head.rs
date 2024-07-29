// use crate::{Message, PipelineData};
//
// pub struct PipelineHeadImpl<Input: PipelineData> {
//     tx: flume::Sender<Message<Input>>,
// }
//
// impl<Input: PipelineData> PipelineHeadImpl<Input> {
//     pub(crate) fn new(tx: flume::Sender<Message<Input>>) -> Self {
//         Self { tx }
//     }
// }
