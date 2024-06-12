//! This module provides some wrappers for [`crate::Filesystem`] that implement some special
//! behavior: actions that are executed after the files are closed, streaming,...
mod async_file_drop;

pub use async_file_drop::{call_sync_all, AsyncFileDropImpl};
