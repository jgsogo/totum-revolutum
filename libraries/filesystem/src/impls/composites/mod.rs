//! This module provides some helpers implementing workflows that involve several [`Filesystem`]s,
//! however, this helpers implement the same [`Filesystem`] interface, so they can be used
//! wherever a [`Filesystem`] can be used.

mod indexed;
mod mirror;

pub use indexed::FilesystemIndexed;
pub use mirror::FilesystemMirror;
