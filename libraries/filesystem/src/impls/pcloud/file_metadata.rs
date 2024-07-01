use crate::{FileMetadata, FilePath, FilePathBuf, Result};

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    path: FilePathBuf,
    hash: String,
    size: u64,
}

impl FileMetadata for RemoteMetadata {
    fn path(&self) -> &FilePath {
        &self.path
    }

    fn size(&self) -> Result<u64> {
        Ok(self.size)
    }

    fn hash(&self) -> Result<String> {
        Ok(self.hash.clone())
    }
}

impl RemoteMetadata {
    pub fn new(path: FilePathBuf, hash: String, size: u64) -> Self {
        Self { path, hash, size }
    }
}
