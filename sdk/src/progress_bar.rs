pub trait ProgressBarBuilder: Send + Sync {
    fn new(&self, _total_size: u64) -> Box<dyn ProgressBar> {
        Box::new(NoProgressBar::default())
    }
}

pub trait ProgressBar: Send {
    fn set_message(&self, message: &str);

    fn set_position(&self, position: u64);

    fn finish_with_message(&self, message: &str);
}

#[derive(Default)]
struct NoProgressBar;

impl ProgressBar for NoProgressBar {
    fn set_message(&self, _message: &str) {}

    fn set_position(&self, _position: u64) {}

    fn finish_with_message(&self, _message: &str) {}
}
