use anyhow::Result;
use async_trait::async_trait;

use super::file_metadata::FileMetadata;

/// Represents the local or remote storage as a filesystem
#[async_trait]
pub trait Filesystem
where
    Self: Sync,
{
    type Metadata: FileMetadata;

    async fn walk_directory(&self, tx: flume::Sender<Self::Metadata>, threads: usize) -> Result<()>;
}
