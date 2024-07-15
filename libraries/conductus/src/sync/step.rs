pub trait PipelineStep<Input, Output>: Send {
    fn process<I: IntoIterator<Item = Input>>(self, rx: I, tx: flume::Sender<Output>);
}
