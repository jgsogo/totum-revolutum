use std::path::Path;

use pcloud_sdk::structures::Metadata;

use crate::diff::basepoint::FileMetadata;

pub trait RemoteFileMetadata: FileMetadata {
    fn from_pcloud_metadata(path: &Path, metadata: Metadata) -> Self;
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
}
