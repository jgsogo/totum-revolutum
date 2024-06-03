//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

pub use error::{Error, Result};
pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem::Filesystem;

mod file;
mod file_metadata;
mod filesystem;
pub mod utils;

pub mod actions;

#[cfg(feature = "test_utils")]
pub mod mocks;

#[cfg(feature = "local")]
pub mod local;

#[cfg(feature = "diff")]
pub mod diff;
pub mod error;
