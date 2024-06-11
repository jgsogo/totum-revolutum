//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

pub use error::{Error, Result};
pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem::{Filesystem, FilesystemRead, FilesystemReadAndWrite, FilesystemRemove, FilesystemWrite};
pub use filesystem_async_drop::FilesystemAsyncDrop;

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
mod filesystem_async_drop;

pub mod mirror;

pub mod indexed;
