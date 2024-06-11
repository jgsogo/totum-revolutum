//! This module provides different [`super::Filesystem`] implementations

#[cfg(feature = "local")]
pub use local::FilesystemLocal;
#[cfg(feature = "local_temp")]
pub use local_temp::FilesystemLocalTemp;

pub mod composites;
#[cfg(feature = "local")]
mod local;
#[cfg(feature = "local_temp")]
mod local_temp;
