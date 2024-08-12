use std::fmt::Debug;

/// A wrapper over the data sent in the pipeline.
#[derive(Clone, PartialEq, Debug)]
pub enum Message<Data: PipelineData> {
    Data(Data),
    Stop(String),
}

/// All the traits that must fulfill anything sent through a pipeline in `conductus` library
pub trait PipelineData: Send + 'static + Debug {}

impl<T: Send + 'static + Debug> PipelineData for T {}
