//! This module provides some helpers implementing workflows that involve several [`crate::Filesystem`]s,
//! however, this helpers implement the same [`crate::Filesystem`] interface, so they can be used
//! wherever a [`crate::Filesystem`] can be used.

#[cfg(feature = "diesel")]
pub use diesel_indexed::FilesystemIndexedDB;
pub use filesystem_backup::FilesystemBackup;
pub use indexed::FilesystemIndexed;

mod filesystem_backup;

#[cfg(feature = "diesel")]
mod diesel_indexed;
mod indexed;
