use crate::{Message, PipelineData};
use flume::Sender;
use tracing::debug;

use super::PipelineStepSync;

/// A [`PipelineStepSync`] that does nothing
#[derive(Default)]
pub struct PipelineStepNoop;

impl<Input: PipelineData> PipelineStepSync<Input, Input> for PipelineStepNoop {
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<Input>>) {
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

    #[test]
    fn test_step_buffer() {
        let step = PipelineStepNoop::default();

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || step.run(0..10, tx));

        let r = rx
            .into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}
