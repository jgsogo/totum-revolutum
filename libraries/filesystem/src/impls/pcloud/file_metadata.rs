use crate::{FileMetadata, FilePath, FilePathBuf};

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

    fn size(&self) -> u64 {
        self.size
    }

    fn hash(&self) -> &str {
        &self.hash
    }
}

impl RemoteMetadata {
    pub fn new(path: FilePathBuf, hash: String, size: u64) -> Self {
        Self { path, hash, size }
    }
}
