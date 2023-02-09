use std::path::{Path, PathBuf};

use pcloud_sdk::structures::Metadata;
use pcloud_sdk::types::FileID;

use filesystem::FileMetadata;

pub trait RemoteFileMetadata: FileMetadata {
    fn from_pcloud_metadata(path: &Path, metadata: Metadata) -> Self;

    fn fileid(&self) -> &FileID;
}

pub type RemoteMetadataEntry = (PathBuf, Metadata);

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    id: String,
    metadata: Metadata,
}

impl FileMetadata for RemoteMetadata {
    type DirEntry = RemoteMetadataEntry;
    fn id(&self) -> &str {
        &self.id
    }
    fn size(&self) -> u64 {
        *self.metadata.size.as_ref().unwrap()
    }

    fn hash(&self) -> String {
        self.metadata.hash.as_ref().unwrap().to_string()
    }
}

impl From<RemoteMetadataEntry> for RemoteMetadata {
    fn from(entry: RemoteMetadataEntry) -> Self {
        let (path, metadata) = entry;
        Self {
            id: path.to_str().unwrap().to_string(),
            metadata,
        }
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
