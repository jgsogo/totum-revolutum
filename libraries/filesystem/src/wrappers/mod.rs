//! This module provides some wrappers for [`crate::Filesystem`] that implement some special
//! behavior: actions that are executed after the files are closed, streaming,...
mod filesystem_async_drop;

pub use filesystem_async_drop::FilesystemAsyncDrop;
