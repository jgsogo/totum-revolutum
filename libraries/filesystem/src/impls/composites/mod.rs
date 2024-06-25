//! This module provides some helpers implementing workflows that involve several [`crate::Filesystem`]s,
//! however, this helpers implement the same [`crate::Filesystem`] interface, so they can be used
//! wherever a [`crate::Filesystem`] can be used.

pub use filesystem_backup::FilesystemBackup;
#[cfg(feature = "diesel_indexed_impl")]
pub use indexed::new_filesystem_indexed_with_db;
pub use indexed::{
    FilesystemIndexed, FilesystemIndexedDatabase, FilesystemIndexedDbDirectory, FilesystemIndexedDbFile,
};

mod filesystem_backup;

pub mod indexed;
