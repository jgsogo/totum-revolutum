pub trait ProgressBarBuilder: Send + Sync {
    fn new(&self, total_size: u64) -> Box<dyn ProgressBar>;
}

pub trait ProgressBar: Send {
    fn set_message(&self, message: &str);

    fn set_position(&self, position: u64);

    fn finish_with_message(&self, message: &str);
}
