use anyhow::{anyhow, Result};

use super::file_metadata::FileMetadata;

/// Represents the local or remote storage as a filesystem
pub trait Filesystem
where
    Self: Sync,
{
    type Metadata: FileMetadata;
}
