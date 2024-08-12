//! Implementations of [`crate::Filesystem`] using different backends, typically involving several
//! storage solutions.

pub use filesystem_backup::FilesystemBackup;
#[cfg(feature = "diesel_indexed_impl")]
pub use indexed::new_filesystem_indexed_with_db;
pub use indexed::{
    FilesystemIndexed, FilesystemIndexedDatabase, FilesystemIndexedDbDirectory, FilesystemIndexedDbFile,
};

mod filesystem_backup;

pub mod indexed;
