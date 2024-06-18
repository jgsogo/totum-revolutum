//! This module provides some helpers implementing workflows that involve several [`crate::Filesystem`]s,
//! however, this helpers implement the same [`crate::Filesystem`] interface, so they can be used
//! wherever a [`crate::Filesystem`] can be used.

pub use filesystem_backup::FilesystemBackup;
#[cfg(feature = "diesel")]
pub use indexed::FilesystemIndexedDB;
pub use indexed::{
    FilesystemIndexed, FilesystemIndexedDatabase, FilesystemIndexedDbDirectory, FilesystemIndexedDbFile,
};

mod filesystem_backup;

mod indexed;
