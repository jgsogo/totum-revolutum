//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

mod file;
mod file_metadata;
mod filesystem;

pub use filesystem::Filesystem;
mod utils;
pub use file::File;
pub use file_metadata::FileMetadata;
