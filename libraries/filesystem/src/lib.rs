//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

pub use error::{Error, Result};
pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem::{Filesystem, FilesystemInnerOperations, FilesystemRead, FilesystemRemove, FilesystemWrite};

mod file;
mod file_metadata;
mod filesystem;
pub mod utils;

pub mod actions;

#[cfg(feature = "diff")]
pub mod diff;
pub mod error;

pub mod impls;
pub mod wrappers;
