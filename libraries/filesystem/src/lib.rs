//! Declares an abstraction over a filesystem, optionally provides an implementation for local filesystem
//!

mod file;
mod file_metadata;
mod filesystem;
mod utils;

pub use file::File;
pub use file_metadata::FileMetadata;
pub use filesystem::Filesystem;
