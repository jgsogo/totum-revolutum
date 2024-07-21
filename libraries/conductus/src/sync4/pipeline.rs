use super::steps::{PipelineStep, PipelineStepMap};
use flume::SendError;
use std::thread::JoinHandle;

#[derive(Clone)]
pub enum Message<Data> {
    Data(Data),
    Close,
}
pub struct Pipeline<Input, Output> {
    tx: flume::Sender<Message<Input>>,
    rx: flume::Receiver<Message<Output>>,
}

impl<Input> Pipeline<Input, Input> {
    pub fn empty(cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        Self { tx, rx }
    }
}

impl<Input: Send + 'static, Output: Send + 'static> Pipeline<Input, Output> {
    pub fn pipe<NextOutput: Send + 'static, PS: PipelineStep<Output, NextOutput> + Send + 'static>(
        self,
        step: PS,
        cap: usize,
    ) -> Pipeline<Input, NextOutput> {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || step.run(self.rx, tx));
        Pipeline { tx: self.tx, rx }
    }

    pub fn send(&self, input: Input) -> Result<(), SendError<Message<Input>>> {
        self.tx.send(Message::Data(input))
    }

    pub fn send_batch<I: IntoIterator<Item = Input> + Send + 'static>(
        &self,
        input: I,
    ) -> JoinHandle<Result<(), SendError<Message<Input>>>> {
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let r: Result<Vec<_>, _> = input.into_iter().map(|it| tx.send(Message::Data(it))).collect();
            r.map(|_| ())
        })
    }

    pub fn map<NextOutput: Send + 'static, Func: Fn(Output) -> NextOutput + Send + 'static>(
        self,
        step: Func,
        cap: usize,
    ) -> Pipeline<Input, NextOutput> {
        let step: PipelineStepMap<Output, NextOutput, Func> = step.into();
        self.pipe(step, cap)
    }

    pub fn buffer(self, cap: usize) -> Self {
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || {
            for it in self.rx {
                tx.send(it).unwrap()
            }
        });
        Self { tx: self.tx, rx }
    }
}

impl<Input: Send + 'static, Output: Clone + Send + 'static> Pipeline<Input, Output> {
    pub fn split(self, cap_lhs: usize, cap_rhs: usize) -> (Pipeline<Input, Output>, Pipeline<Input, Output>) {
        let (tx_lhs, rx_lhs) = flume::bounded(cap_lhs);
        let (tx_rhs, rx_rhs) = flume::bounded(cap_rhs);

        std::thread::spawn(move || {
            for it in self.rx {
                tx_lhs.send(it.clone()).unwrap();
                tx_rhs.send(it.clone()).unwrap();
            }
        });

        let lhs = Self {
            tx: self.tx.clone(),
            rx: rx_lhs,
        };
        let rhs = Self {
            tx: self.tx.clone(),
            rx: rx_rhs,
        };

        (lhs, rhs)
    }
}

impl<Input, Output> IntoIterator for Pipeline<Input, Output> {
    type Item = Output;
    type IntoIter = PipelineIter<Input, Output>; //<flume::Receiver<Output> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        PipelineIter { pipeline: self }
    }
}

pub struct PipelineIter<Input, Output> {
    pipeline: Pipeline<Input, Output>,
}

impl<Input, Output> Iterator for PipelineIter<Input, Output> {
    type Item = Output;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pipeline.rx.recv() {
            Ok(msg) => match msg {
                Message::Data(data) => Some(data),
                Message::Close => None,
            },
            Err(_) => None,
        }
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
    fn test_split() {
        let pipeline = Pipeline::empty(2); //.map(|input: i32| input * 2, 2);
        let (p_lhs, p_rhs) = pipeline.split(2, 2);
        p_lhs.send_batch(0..3);
        p_rhs.send_batch(0..2);

        let out_lhs = p_lhs.into_iter().collect::<Vec<_>>();
        let out_rhs = p_rhs.into_iter().collect::<Vec<_>>();

        assert_eq!(out_lhs, vec![1]);
        assert_eq!(out_rhs, vec![2]);
    }
}
