//! This module provides different [`super::Filesystem`] implementations

#[cfg(feature = "local")]
mod local;

#[cfg(feature = "local")]
pub use local::FilesystemLocal;
