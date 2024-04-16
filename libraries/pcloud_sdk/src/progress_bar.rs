use tracing::{info, trace};

/// A builder for [`ProgressBar`] items. All bars should be created
/// using the same builder so it can control the output to the
/// terminal.
///
/// By default it creates a mock that will print just a trace message.
pub trait ProgressBarBuilder: Send + Sync {
    fn build(&self, _total_size: u64) -> Box<dyn ProgressBar> {
        Box::<NoProgressBar>::default()
    }
}

/// Represents a progress bar
pub trait ProgressBar: Send {
    /// Message to show together with the progress bar
    fn set_message(&self, message: &str);

    /// Level of completion of the progress bar, same units as the
    /// ones used to create it
    fn set_position(&self, position: u64);

    /// Completes the progress bar and set the given message
    fn finish_with_message(&self, message: &str);

    /// Completes the progress bar keeping existing message
    fn finish(&self);
}

#[derive(Default)]
pub struct NoProgressBar;

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

// #[cfg(feature = "indicatif")]
// impl ProgressBar for indicatif::ProgressBar {
//     fn set_message(&self, message: &str) {
//         self.set_message(message.to_string());
//     }

//     fn set_position(&self, position: u64) {
//         self.set_position(position);
//     }

//     fn finish_with_message(&self, message: &str) {
//         self.finish_with_message(message.to_string());
//     }

//     fn finish(&self) {
//         self.finish();
//     }
// }
