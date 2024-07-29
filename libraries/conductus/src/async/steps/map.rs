use std::future::Future;
use std::marker::PhantomData;

use async_trait::async_trait;
use flume::Sender;
use futures::{pin_mut, Stream};
use tokio_stream::StreamExt;
use tracing::debug;

use crate::{Message, PipelineData};

use super::PipelineStepAsync;

pub struct PipelineStepMap<Input: PipelineData + Sync, Output: PipelineData, Fut, F>
where
    F: Fn(Input) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    func: F,
    _input: PhantomData<Input>,
}

impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> PipelineStepMap<Input, Output, Fut, F>
where
    F: Fn(Input) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    pub async fn map(&self, input: Input) -> Output {
        (self.func)(input).await
    }
}

#[async_trait]
impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> PipelineStepAsync<Input, Output>
    for PipelineStepMap<Input, Output, Fut, F>
where
    F: Fn(Input) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: Sender<Message<Output>>) {
        pin_mut!(source);
        while let Some(it) = source.next().await {
            let out = self.map(it).await;
            if let Err(e) = target.send_async(Message::Data(out)).await {
                debug!("Error sending from blanket implementation of PipelineStepMap: {e}");
            }
        }
    }
}

impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> From<F> for PipelineStepMap<Input, Output, Fut, F>
where
    F: Fn(Input) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    fn from(value: F) -> Self {
        Self {
            func: value,
            _input: PhantomData,
        }
    }
}

impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> Clone for PipelineStepMap<Input, Output, Fut, F>
where
    F: 'static + Fn(Input) -> Fut + Send + Sync + Clone,
    Fut: 'static + Future<Output = Output> + Send,
{
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            _input: self._input,
        }
    }
}

impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> Copy for PipelineStepMap<Input, Output, Fut, F>
where
    F: Fn(Input) -> Fut + Send + Sync + 'static + Copy,
    Fut: Future<Output = Output> + Send + 'static,
{
}

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    async fn double(input: i32) -> i32 {
        input * 2
    }

    #[tokio::test]
    async fn test_step_map() {
        let step = PipelineStepMap::from(double);

        assert_eq!(step.map(2).await, 4);
        assert_eq!(step.map(1).await, 2);
        assert_eq!(step.map(0).await, 0);

        let (tx, rx) = flume::bounded(2);
        tokio::spawn(async move { step.run(stream::iter(0..10), tx).await });

        let r = rx
            .into_stream()
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .filter_map(|v| match v {
                Message::Data(d) => Some(d),
                Message::Stop(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(r, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18])
    }
}
