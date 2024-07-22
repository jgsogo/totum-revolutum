use std::collections::VecDeque;
use std::thread::JoinHandle;

use flume::SendError;

use crate::sync4::tail::PipelineTailIter;
use crate::sync4::{PipelineHead, PipelineTail};

#[derive(Clone)]
pub enum Message<Data> {
    Data(Data),
    Flush,
}
pub struct Pipeline<Input, Output> {
    head: PipelineHead<Input>,
    tail: PipelineTail<Output>,
}

impl<Input: Send + 'static> Pipeline<Input, Input> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self {
            head: PipelineHead::new(tx),
            tail: PipelineTail::new(rx),
        }
    }
}

impl<Input: Send + 'static, Output: Send + 'static> Pipeline<Input, Output> {
    pub fn ends(self) -> (PipelineHead<Input>, PipelineTail<Output>) {
        let Pipeline { head, tail } = self;
        (head, tail)
    }

    pub fn send(&self, input: Input) -> Result<(), SendError<Input>> {
        self.head.send(input)
    }

    pub fn send_batch<I: IntoIterator<Item = Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Input>>> {
        self.head.send_batch(input)
    }

    pub fn map<NextOutput: Send + 'static, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        step: Func,
        cap: usize,
    ) -> Pipeline<Input, NextOutput> {
        let tail = self.tail.map(step, cap);
        Pipeline { head: self.head, tail }
    }

    pub fn buffer(self, cap: usize) -> Self {
        let tail = self.tail.buffer(cap);
        Pipeline { head: self.head, tail }
    }

    pub fn drain(self) -> PipelineTailIter<Output> {
        drop(self.head);
        self.tail.into_iter()
    }

    pub fn window<NextOutput: Send + 'static, Func: Fn(&VecDeque<Output>) -> NextOutput + Send + 'static>(
        self,
        func: Func,
        window_size: usize,
        cap: usize,
    ) -> Pipeline<Input, NextOutput> {
        let tail = self.tail.window(func, window_size, cap);
        Pipeline { head: self.head, tail }
    }
}

impl<Input, Output> IntoIterator for Pipeline<Input, Output> {
    type Item = Output;
    type IntoIter = PipelineTailIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        self.tail.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        let pipeline = Pipeline::empty(2);

        pipeline.send(3).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![3]);
    }

    #[test]
    fn test_send_batch() {
        let pipeline = Pipeline::empty(2);

        pipeline.send_batch(0..3);
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2]);
    }

    #[test]
    fn test_map() {
        let pipeline = Pipeline::empty(2).map(|input: i32| input * 2, 2);

        pipeline.send(3).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![6]);
    }

    #[test]
    fn test_buffer() {
        let pipeline = Pipeline::empty(2).buffer(1);

        pipeline.send(0).unwrap();
        pipeline.send(1).unwrap();
        pipeline.send(2).unwrap();
        let out = pipeline.into_iter().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1, 2]);
    }

    #[test]
    fn test_drain() {
        let pipeline = Pipeline::empty(2);

        pipeline.send(0).unwrap();
        pipeline.send(1).unwrap();
        let out = pipeline.drain().collect::<Vec<_>>();
        assert_eq!(out, vec![0, 1]);
    }

    #[test]
    fn test_split() {
        let pipeline = Pipeline::empty(2).map(|input: i32| input * 2, 2);
        let (head, tail) = pipeline.ends();
        let (t1, t2) = tail.split();
        let t2 = t2.map(|input| input * 2, 2);

        head.send_batch(0..3);

        drop(head);

        let out_t1 = t1.into_iter().collect::<Vec<_>>();
        let out_t2 = t2.into_iter().collect::<Vec<_>>();

        assert_eq!(out_t1, vec![0, 2, 4]);
        assert_eq!(out_t2, vec![0, 4, 8]);
    }
}
