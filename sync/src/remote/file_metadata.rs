use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;
use tracing::trace;

use pcloud_sdk::id::FileID;
use pcloud_sdk::structures::Metadata;

use crate::actions::{Copy, Remove, Rename};
use crate::diff::basepoint::FileMetadata;
use crate::local::{LocalFileMetadata, LocalMetadata};

pub trait RemoteFileMetadata: FileMetadata {
    fn from_pcloud_metadata(path: &Path, metadata: Metadata) -> Self;

    fn fileid(&self) -> &FileID;
}

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    id: String,
    metadata: Metadata,
}

impl FileMetadata for RemoteMetadata {
    fn id(&self) -> &str {
        &self.id
    }
    fn size(&self) -> u64 {
        self.metadata.size.as_ref().unwrap().clone()
    }

    fn hash(&self) -> String {
        self.metadata.hash.as_ref().unwrap().to_string()
    }
}

impl RemoteFileMetadata for RemoteMetadata {
    fn from_pcloud_metadata(path: &Path, metadata: Metadata) -> Self {
        Self {
            id: path.to_string_lossy().parse().unwrap(),
            metadata,
        }
    }

    fn fileid(&self) -> &FileID {
        self.metadata.fileid.as_ref().unwrap()
    }
}

#[async_trait]
impl Copy<LocalMetadata> for RemoteMetadata {
    async fn copy(self, rhs: Option<LocalMetadata>) -> Result<(Self, LocalMetadata)> {
        match rhs {
            Some(rhs) => {
                trace!(
                    "Copy from remote '{}' to local '{}' (override)",
                    self.fileid(),
                    rhs.path().display()
                );
            }
            None => {
                trace!("Copy from remote '{}' to local ' (new file)", self.fileid(),);
            }
        }
        Ok((self, rhs.unwrap()))
    }
}

#[async_trait]
impl Copy<RemoteMetadata> for LocalMetadata {
    async fn copy(self, rhs: Option<RemoteMetadata>) -> Result<(Self, RemoteMetadata)> {
        trace!(
            "Copy from local '{}' to remote '{}'",
            self.path().display(),
            rhs.as_ref().map_or("".to_string(), |v| v.fileid().to_string())
        );
        Ok((self, rhs.unwrap()))
    }
}

#[async_trait]
impl Remove for RemoteMetadata {
    async fn remove(self) -> Result<()> {
        trace!("Remove remote '{}'", self.fileid());
        Ok(())
    }
}

#[async_trait]
impl Rename for RemoteMetadata {
    async fn rename(self) -> Result<Self> {
        trace!("Rename remote '{}'", self.fileid());
        Ok(self)
    }
}
