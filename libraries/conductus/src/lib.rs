pub use error::{Error, Result};
pub use r#async::PipelineAsync;
pub use sync::Pipeline as PipelineSync;

pub mod error;

pub mod r#async;

pub mod sync;
