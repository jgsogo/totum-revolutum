use tracing::{info, trace};

pub trait ProgressBarBuilder: Send + Sync {
    fn new(&self, _total_size: u64) -> Box<dyn ProgressBar> {
        Box::new(NoProgressBar::default())
    }
}

pub trait ProgressBar: Send {
    fn set_message(&self, message: &str);

    fn set_position(&self, position: u64);

    fn finish_with_message(&self, message: &str);
    fn finish(&self);
}

#[derive(Default)]
struct NoProgressBar;

impl ProgressBar for NoProgressBar {
    fn set_message(&self, message: &str) {
        info!("NoProgressBar::set_message({})", message);
    }

    fn set_position(&self, position: u64) {
        trace!("NoProgressBar::set_position({})", position)
    }

    fn finish_with_message(&self, message: &str) {
        info!("NoProgressBar::finish_with_message({})", message)
    }

    fn finish(&self) {
        info!("NoProgressBar::finish()")
    }
}

#[cfg(feature = "indicatif")]
impl ProgressBar for indicatif::ProgressBar {
    fn set_message(&self, message: &str) {
        self.set_message(message.to_string());
    }

    fn set_position(&self, position: u64) {
        self.set_position(position);
    }

    fn finish_with_message(&self, message: &str) {
        self.finish_with_message(message.to_string());
    }

    fn finish(&self) {
        self.finish();
    }
}
