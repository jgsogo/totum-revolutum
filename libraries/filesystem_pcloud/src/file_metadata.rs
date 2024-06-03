use camino::{Utf8Path, Utf8PathBuf};

use filesystem::FileMetadata;
use pcloud_sdk::structures::MetadataFile;

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    relative_path: Utf8PathBuf,
    metadata: MetadataFile,
}

impl FileMetadata for RemoteMetadata {
    fn path(&self) -> &Utf8Path {
        &self.relative_path
    }

    fn size(&self) -> u64 {
        *self.metadata.size.as_ref().unwrap()
    }

    fn hash(&self) -> String {
        self.metadata.hash.as_ref().unwrap().to_string()
    }
}

impl RemoteMetadata {
    pub fn new(relative_path: Utf8PathBuf, metadata: MetadataFile) -> Self {
        Self {
            relative_path,
            metadata,
        }
    }
}
