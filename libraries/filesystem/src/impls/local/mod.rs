//! Implementations of [`crate::Filesystem`] using local storage

use filesystem::FilesystemLocal;

/// Implementation of [`crate::Filesystem`] using async [`async_std::fs::File`] files
pub type FilesystemLocalAsync = FilesystemLocal<async_std::fs::File>;
/// Implementation of [`crate::Filesystem`] using regular [`std::fs::File`] files.
pub type FilesystemLocalSync = FilesystemLocal<std::fs::File>;

mod file;
mod filesystem;
mod parallel_visitor;
