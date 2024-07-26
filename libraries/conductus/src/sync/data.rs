use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug)]
pub enum Message<Data> {
    Data(Data),
    Flush,
}

pub trait PipelineData: Send + 'static + Debug {}

impl<T: Send + 'static + Debug> PipelineData for T {}
