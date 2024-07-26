use std::marker::PhantomData;

use flume::Sender;
use tracing::debug;

use crate::sync::Message;

use super::PipelineStep;

pub struct PipelineStepMap<Input, Output, Func>
where
    Func: Fn(Input) -> Output,
{
    func: Func,
    _input: PhantomData<Input>,
}

impl<Input, Output, Func> PipelineStepMap<Input, Output, Func>
where
    Func: Fn(Input) -> Output,
{
    pub fn map(&self, input: Input) -> Output {
        (self.func)(input)
    }
}

impl<Input, Output, Func> PipelineStep<Input, Output> for PipelineStepMap<Input, Output, Func>
where
    Func: Fn(Input) -> Output,
{
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<Output>>) {
        for it in source {
            let out = self.map(it);
            if let Err(e) = target.send(Message::Data(out)) {
                debug!("Error sending from blanket implementation of PipelineStepMap: {e}");
            }
        }
    }
}

impl<Input, Output, Func> From<Func> for PipelineStepMap<Input, Output, Func>
where
    Func: Fn(Input) -> Output,
{
    fn from(value: Func) -> Self {
        PipelineStepMap {
            func: value,
            _input: PhantomData,
        }
    }
}

impl<Input, Output, Func> Clone for PipelineStepMap<Input, Output, Func>
where
    Func: Fn(Input) -> Output + Clone,
{
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            _input: self._input,
        }
    }
}

impl<Input, Output, Func> Copy for PipelineStepMap<Input, Output, Func> where Func: Fn(Input) -> Output + Copy {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_map() {
        let double = |input: i32| input * 2;
        let step: PipelineStepMap<i32, i32, _> = double.into();

        assert_eq!(step.map(2), 4);
        assert_eq!(step.map(1), 2);
        assert_eq!(step.map(0), 0);

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || step.run(0..10, tx));

        let r = rx
            .into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Flush => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18])
    }
}
