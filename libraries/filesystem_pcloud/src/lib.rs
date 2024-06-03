//! Implements traits from [`filesystem` crate](../filesystem/index.html) for pCloud remote filesystem
//!

pub use file::CHUNK_SIZE;
pub use file_metadata::{RemoteMetadata, RemoteMetadataEntry};

pub use self::filesystem::FilesystemPCloud;

mod file;
mod file_metadata;
mod filesystem;
