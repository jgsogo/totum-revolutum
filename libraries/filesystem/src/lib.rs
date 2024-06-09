//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

pub use error::{Error, Result};
pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem::Filesystem;
pub use filesystem_cloned::FilesystemCloned;

mod file;
mod file_metadata;
mod filesystem;
pub mod utils;

pub mod actions;

#[cfg(feature = "local_temp")]
pub mod local_temp;

#[cfg(feature = "local")]
pub mod local;

#[cfg(feature = "diff")]
pub mod diff;
pub mod error;
mod filesystem_cloned;
