use pcloud_sdk::structures::MetadataFile;
use pcloud_sdk::types::FileID;

use crate::{Error, FileMetadata, FilePath, FilePathBuf, Result};

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    path: FilePathBuf,
    metadata: MetadataFile,
}

impl FileMetadata for RemoteMetadata {
    fn path(&self) -> &FilePath {
        &self.path
    }

    fn size(&self) -> Result<u64> {
        match self.metadata.size {
            None => Err(Error::Other("metadata.size not available".into())),
            Some(s) => Ok(s),
        }
    }

    fn hash(&self) -> Result<String> {
        match self.metadata.hash {
            None => Err(Error::Other("metadata.hash not available".into())),
            Some(h) => Ok(h.to_string()),
        }
    }
}

impl RemoteMetadata {
    pub fn new(path: FilePathBuf, metadata: MetadataFile) -> Self {
        Self { path, metadata }
    }

    #[allow(dead_code)]
    pub fn fileid(&self) -> &FileID {
        &self.metadata.fileid
    }
}
