use std::collections::VecDeque;
use std::marker::PhantomData;

use flume::Sender;
use tracing::{debug, warn};

use crate::sync::{Message, PipelineData};

use super::PipelineStep;

pub struct PipelineStepWindow<Input: PipelineData, Output: PipelineData, Func>
where
    Func: Fn(&VecDeque<Input>) -> Output + Send + 'static,
{
    window_size: usize,
    func: Func,
    _input: PhantomData<Input>,
}

impl<Input: PipelineData, Output: PipelineData, Func> PipelineStepWindow<Input, Output, Func>
where
    Func: Fn(&VecDeque<Input>) -> Output + Send + 'static,
{
    pub fn window(&self, input: &VecDeque<Input>) -> Output {
        (self.func)(input)
    }
}

impl<Input: PipelineData, Output: PipelineData, Func> PipelineStep<Input, Output>
    for PipelineStepWindow<Input, Output, Func>
where
    Func: Fn(&VecDeque<Input>) -> Output + Send + 'static,
{
    fn run<I: IntoIterator<Item = Input>>(&self, source: I, target: Sender<Message<Output>>) {
        let mut iter = source.into_iter();

        // Collect items until the window is full
        let mut w = VecDeque::with_capacity(self.window_size);
        while w.len() != self.window_size {
            match iter.next() {
                None => {
                    warn!("Iterator exhausted before filling the first window");
                    return;
                }
                Some(it) => {
                    w.push_back(it);
                }
            }
        }

        // Permanent flow
        let out = self.window(&w);
        if let Err(e) = target.send(Message::Data(out)) {
            debug!("Error sending from blanket implementation of PipelineStepWindow: {e}");
        }
        for it in iter {
            w.pop_front();
            w.push_back(it);
            let out = self.window(&w);
            if let Err(e) = target.send(Message::Data(out)) {
                debug!("Error sending from blanket implementation of PipelineStepWindow: {e}");
            }
        }
    }
}

impl<Input: PipelineData, Output: PipelineData, Func> PipelineStepWindow<Input, Output, Func>
where
    Func: Fn(&VecDeque<Input>) -> Output + Send + 'static,
{
    pub fn new(func: Func, window_size: usize) -> Self {
        Self {
            window_size,
            func,
            _input: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window() {
        let double = |input: &VecDeque<i32>| input.iter().cloned().collect::<Vec<_>>();
        let step = PipelineStepWindow::new(double, 3);

        let vec = VecDeque::from([0, 1, 2]);
        assert_eq!(step.window(&vec), vec![0, 1, 2]);

        let (tx, rx) = flume::bounded(2);
        std::thread::spawn(move || step.run(0..6, tx));

        let r = rx
            .into_iter()
            .filter_map(|it| match it {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]])
    }
}
