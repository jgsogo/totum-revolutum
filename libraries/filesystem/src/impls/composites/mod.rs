//! This module provides some helpers implementing workflows that involve several [`crate::Filesystem`]s,
//! however, this helpers implement the same [`crate::Filesystem`] interface, so they can be used
//! wherever a [`crate::Filesystem`] can be used.

mod filesystem_backup;
pub use filesystem_backup::FilesystemBackup;
