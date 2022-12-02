use std::path::Path;

use pcloud_sdk::id::FileID;
use pcloud_sdk::structures::Metadata;

use crate::diff::filesystem::FileMetadata;

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
        *self.metadata.size.as_ref().unwrap()
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
