//! This module provides different [`super::Filesystem`] implementations

#[cfg(feature = "local")]
pub use local::FilesystemLocal;
#[cfg(feature = "local_temp")]
pub use local_temp::FilesystemLocalTemp;
#[cfg(feature = "pcloud")]
pub use pcloud::FilesystemPCloud;

pub mod composites;
#[cfg(feature = "local")]
mod local;
#[cfg(feature = "local_temp")]
mod local_temp;

#[cfg(feature = "pcloud")]
mod pcloud;

#[cfg(all(feature = "pcloud", feature = "test_utils"))]
pub use pcloud::CHUNK_SIZE as PCLOUD_CHUNK_SIZE;
