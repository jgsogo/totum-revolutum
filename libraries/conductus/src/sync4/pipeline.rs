use crate::sync4::tail::PipelineTailIter;
use crate::sync4::{PipelineHead, PipelineTail};
use flume::SendError;
use std::thread::JoinHandle;

#[derive(Clone)]
pub enum Message<Data> {
    Data(Data),
    Close,
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

    // #[test]
    // fn test_split() {
    //     let pipeline = Pipeline::empty(2); //.map(|input: i32| input * 2, 2);
    //     let (p_lhs, p_rhs) = pipeline.split(2, 2);
    //     p_lhs.send_batch(0..3);
    //     p_rhs.send_batch(0..2);
    //
    //     let out_lhs = p_lhs.into_iter().collect::<Vec<_>>();
    //     let out_rhs = p_rhs.into_iter().collect::<Vec<_>>();
    //
    //     assert_eq!(out_lhs, vec![1]);
    //     assert_eq!(out_rhs, vec![2]);
    // }
}
