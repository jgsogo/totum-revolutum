use std::path::Path;

use anyhow::{anyhow, Result};

use super::file_metadata::FileMetadata;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SnapshotStatus {
    ToBeDeleted,
    New,
    Modified,
    Idle,
}

/// Represents the local or remote storage as a filesystem
pub trait Filesystem
where
    Self: Sync,
{
    type Metadata: FileMetadata;

    fn tx(&self) -> &flume::Sender<Self::Metadata>;

    fn file_found(&self, entry: <<Self as Filesystem>::Metadata as FileMetadata>::DirEntry) -> Result<()> {
        let data: Self::Metadata = entry.into();
        self.tx().send(data).map_err(|e| anyhow!("Error sending metadata: {e}"))
    }
}
