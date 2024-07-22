use std::collections::VecDeque;

use crate::sync4::pipeline::Message;
use crate::sync4::steps::{PipelineStep, PipelineStepMap, PipelineStepNoop, PipelineStepWindow};

pub struct PipelineTail<Output> {
    rx: flume::Receiver<Message<Output>>,
}

impl<Output: Send + 'static> PipelineTail<Output> {
    pub(crate) fn new(rx: flume::Receiver<Message<Output>>) -> Self {
        Self { rx }
    }

    pub fn pipe<NextOutput: Send + 'static, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> PipelineTail<NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || step.run(self, tx));
        PipelineTail::new(rx)
    }

    pub fn map<NextOutput: Send + 'static, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        step: Func,
        cap: usize,
    ) -> PipelineTail<NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = step.into();
        self.pipe(step, cap)
    }

    pub fn buffer(self, cap: usize) -> Self {
        let step = PipelineStepNoop;
        self.pipe(step, cap)
    }

    pub fn window<NextOutput: Send + 'static, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> PipelineTail<NextOutput> {
        let step = PipelineStepWindow::new(func, window_size);
        self.pipe(step, cap)
    }
}

impl<Output: Clone + Send + 'static> PipelineTail<Output> {
    pub fn split(self) -> (PipelineTail<Output>, PipelineTail<Output>) {
        let (tx_lhs, rx_lhs) = flume::unbounded();
        let (tx_rhs, rx_rhs) = flume::unbounded();

        std::thread::spawn(move || {
            for it in self.rx {
                tx_lhs.send(it.clone()).unwrap();
                tx_rhs.send(it.clone()).unwrap();
            }
        });

        let tail_lhs = PipelineTail::new(rx_lhs);
        let tail_rhs = PipelineTail::new(rx_rhs);
        (tail_lhs, tail_rhs)
    }
}

impl<Output> IntoIterator for PipelineTail<Output> {
    type Item = Output;
    type IntoIter = PipelineTailIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        PipelineTailIter { tail: self }
    }
}

pub struct PipelineTailIter<Output> {
    tail: PipelineTail<Output>,
}

impl<Output> Iterator for PipelineTailIter<Output> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tail.rx.recv() {
            Ok(msg) => match msg {
                Message::Data(data) => Some(data),
                Message::Flush => None,
            },
            Err(_) => None,
        }
    }
}
