use log::debug;

use crate::sync::step::PipelineStep;

/// Implementation of a sync pipeline
pub struct Pipeline<Output> {
    rx: flume::Receiver<Output>,
    cap: usize,
}

impl<Output: Send + 'static> Pipeline<Output> {
    /// Creates a new [`Pipeline`] from a blocking function
    ///
    /// # Example
    ///
    /// ```
    /// use conductus::Pipeline;
    ///
    /// let pl = Pipeline::new(|tx|{
    ///     for it in 0..10 {
    ///         tx.send(it).unwrap();
    ///     }
    /// }, 2);
    /// assert_eq!(pl.into_iter().collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
    /// ```
    pub fn new<F>(func: F, cap: usize) -> Self
    where
        F: FnOnce(flume::Sender<Output>) + Send + 'static,
    {
        let (tx, rx) = flume::bounded(cap);
        // For blocking functions, we need to use std::thread, otherwise the sender will block
        // if there is no capacity left in the channel, and all the asynchronous runtime will be
        // blocked.
        std::thread::spawn(move || func(tx));
        Self { rx, cap }
    }

    /// Creates a new [`Pipeline`] from an iterator
    ///
    /// # Example
    ///
    /// ```
    /// use conductus::Pipeline;
    ///
    /// let pl = Pipeline::from(0..10, 5);
    /// assert_eq!(pl.into_iter().collect::<Vec<_>>(), (0..10).collect::<Vec<_>>());
    /// ```
    pub fn from<I: IntoIterator<Item = Output> + Send + 'static>(source: I, cap: usize) -> Self {
        Self::new(
            move |tx| {
                for it in source {
                    if let Err(e) = tx.send(it) {
                        debug!("All receivers have been dropped: {e}");
                    }
                }
            },
            cap,
        )
    }

    pub fn then<StepOutput: Send + 'static, PS: PipelineStep<Output, StepOutput> + 'static>(
        self,
        step: PS,
    ) -> Pipeline<StepOutput> {
        self.pipe(move |rx, tx| step.process(rx, tx))
    }

    /// Pipes one operation after an existing pipeline
    ///
    /// # Example
    ///
    /// ```
    /// use tracing::error;
    /// use conductus::Pipeline;
    ///
    /// let pl = Pipeline::from(0..4, 2).pipe(|input, tx: flume::Sender<i32>| {
    ///     for it in input {
    ///         tx.send(-2 * it).unwrap();
    ///     }
    /// });
    ///
    /// assert_eq!(pl.into_iter().collect::<Vec<_>>(), vec![0, -2, -4, -6]);
    /// ```
    ///
    pub fn pipe<StepOutput: Send + 'static, Func>(self, func: Func) -> Pipeline<StepOutput>
    where
        Func: FnOnce(flume::IntoIter<Output>, flume::Sender<StepOutput>) + Send + 'static,
    {
        let cap = self.cap;
        let (tx, rx) = flume::bounded(cap);
        std::thread::spawn(move || {
            func(self.into_iter(), tx);
        });

        Pipeline { rx, cap }
    }

    /// Consumes the pipeline without collecting results
    pub fn drain(self) {
        for _ in self {}
    }
}

impl<Output> IntoIterator for Pipeline<Output> {
    type Item = Output;
    type IntoIter = <flume::Receiver<Output> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rx.into_iter()
    }
}

#[cfg(test)]
mod tests {}
