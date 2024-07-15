pub use error::{Error, Result};
pub use r#async::PipelineAsync;
pub use sync::Pipeline;

pub mod error;

pub mod r#async;
pub mod sync;
