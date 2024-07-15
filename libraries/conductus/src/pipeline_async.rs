use std::future::Future;

use flume::r#async::RecvStream;

pub struct PipelineAsync<Output> {
    rx: flume::Receiver<Output>,
    _cap: usize,
}

impl<Output: Send + 'static> PipelineAsync<Output> {
    /// Creates a new [`PipelineAsync`] from an async function
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::time::{sleep, Duration};
    /// use conductus::PipelineAsync;
    /// use futures::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let pl = PipelineAsync::new(
    ///         move |tx| async move {
    ///             for it in 0..10 {
    ///                 tx.send_async(it).await.unwrap();
    ///             }
    ///         },
    ///         5,
    ///     );
    ///
    ///     // Collect the results using a stream
    ///     let mut results = Vec::new();
    ///     let mut stream = pl.into_stream();
    ///     while let Some(value) = stream.next().await {
    ///         results.push(value);
    ///     }
    ///     assert_eq!(results, (0..10).collect::<Vec<_>>());
    /// }
    /// ```
    pub fn new<F, Fut>(func: F, cap: usize) -> Self
    where
        F: Fn(flume::Sender<Output>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let (tx, rx) = flume::bounded(cap);
        tokio::spawn(async move {
            func(tx).await;
        });
        Self { rx, _cap: cap }
    }

    pub fn into_stream(self) -> RecvStream<'static, Output> {
        self.rx.into_stream()
    }
}
