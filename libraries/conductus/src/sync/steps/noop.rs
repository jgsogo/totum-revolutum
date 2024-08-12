use crate::common::steps::noop::PipelineStepNoop;
use crate::{Message, PipelineData};
use flume::Sender;
use tracing::debug;

use super::PipelineStepSync;

impl<Input: PipelineData> PipelineStepSync<Input, Input> for PipelineStepNoop {
    fn run<I: Iterator<Item = Input>>(&self, source: I, target: Sender<Message<Input>>) {
        for it in source {
            if let Err(e) = target.send(Message::Data(it)) {
                debug!("Error sending from blanket implementation of PipelineStepBuffer: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::steps::tests::collect_rx;

    #[test]
    fn test_step_buffer() {
        let step = PipelineStepNoop::default();

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || step.run(0..10, tx));

        let r = collect_rx(rx);
        assert_eq!(r, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
