use camino::{Utf8Path, Utf8PathBuf};

use filesystem::FileMetadata;
use pcloud_sdk::structures::MetadataFile;
use pcloud_sdk::types::FileID;

#[allow(dead_code)]
pub trait RemoteFileMetadata: FileMetadata {
    fn from_pcloud_metadata(path: &Utf8Path, metadata: MetadataFile) -> Self;

    fn fileid(&self) -> &FileID;
}

pub type RemoteMetadataEntry = (Utf8PathBuf, MetadataFile);

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    id: String,
    metadata: MetadataFile,
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
            id: path.to_string(),
            metadata,
        }
    }
}

impl RemoteFileMetadata for RemoteMetadata {
    fn from_pcloud_metadata(path: &Utf8Path, metadata: MetadataFile) -> Self {
        Self {
            id: path.to_string(),
            metadata,
        }
    }

    fn fileid(&self) -> &FileID {
        &self.metadata.fileid
    }
}
