use camino::{Utf8Path, Utf8PathBuf};

use filesystem::{FileMetadata, Result};
use pcloud_sdk::structures::MetadataFile;
use pcloud_sdk::types::FileID;

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    relative_path: Utf8PathBuf,
    metadata: MetadataFile,
}

impl FileMetadata for RemoteMetadata {
    fn path(&self) -> &Utf8Path {
        &self.relative_path
    }

    fn size(&self) -> Result<u64> {
        match self.metadata.size {
            None => Err(filesystem::Error::Other("metadata.size not available".into())),
            Some(s) => Ok(s),
        }
    }

    fn hash(&self) -> Result<String> {
        match self.metadata.hash {
            None => Err(filesystem::Error::Other("metadata.size not available".into())),
            Some(h) => Ok(h.to_string()),
        }
    }
}

impl RemoteMetadata {
    pub fn new(relative_path: Utf8PathBuf, metadata: MetadataFile) -> Self {
        Self {
            relative_path,
            metadata,
        }
    }

    pub fn fileid(&self) -> &FileID {
        &self.metadata.fileid
    }
}
