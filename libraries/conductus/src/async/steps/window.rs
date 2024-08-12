use async_trait::async_trait;
use std::collections::VecDeque;
use std::future::Future;
use std::marker::PhantomData;

use flume::Sender;
use futures::{pin_mut, Stream};
use tokio_stream::StreamExt;
use tracing::{debug, warn};

use crate::r#async::steps::PipelineStepAsync;
use crate::{Message, PipelineData};

pub struct PipelineStepWindow<Input: PipelineData + Sync, Output: PipelineData, Fut, F>
where
    F: Fn(VecDeque<Input>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    func: F,
    window_size: usize,
    _input: PhantomData<Input>,
}

impl<Input: PipelineData + Sync, Output: PipelineData, Fut, F> PipelineStepWindow<Input, Output, Fut, F>
where
    F: Fn(VecDeque<Input>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    pub fn new(func: F, window_size: usize) -> Self {
        Self {
            func,
            window_size,
            _input: PhantomData,
        }
    }

    pub async fn window(&self, input: VecDeque<Input>) -> Output {
        (self.func)(input).await
    }
}

#[async_trait]
impl<Input: PipelineData + Sync + Clone, Output: PipelineData, Fut, F> PipelineStepAsync<Input, Output>
    for PipelineStepWindow<Input, Output, Fut, F>
where
    F: Fn(VecDeque<Input>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Output> + Send + 'static,
{
    async fn run<I: Stream<Item = Input> + Send>(&self, source: I, target: Sender<Message<Output>>) {
        // Collect items until the window is full
        let mut w = VecDeque::with_capacity(self.window_size);
        pin_mut!(source);

        while w.len() != self.window_size {
            match source.next().await {
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
        let out = self.window(w.clone()).await;
        if let Err(e) = target.send_async(Message::Data(out)).await {
            debug!("Error sending from implementation of PipelineStepWindow: {e}");
        }
        while let Some(it) = source.next().await {
            w.pop_front();
            w.push_back(it);
            let out = self.window(w.clone()).await;
            if let Err(e) = target.send_async(Message::Data(out)).await {
                debug!("Error sending from implementation of PipelineStepWindow: {e}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#async::steps::tests::collect_rx;
    use futures::stream;

    #[tokio::test]
    async fn test_window() {
        let step = PipelineStepWindow::new(
            move |input: VecDeque<i32>| async move { input.iter().cloned().collect::<Vec<_>>() },
            3,
        );

        let vec = VecDeque::from([0, 1, 2]);
        assert_eq!(step.window(vec).await, vec![0, 1, 2]);

        let (tx, rx) = flume::bounded(2);
        tokio::spawn(async move { step.run(stream::iter(0..6), tx).await });

        let r = collect_rx(rx).await;
        assert_eq!(r, vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]])
    }
}
