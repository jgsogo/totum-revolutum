//! Utilities related to async
pub mod error;
mod side_task;

pub use error::{Error, Result};
pub use side_task::SideTask;
