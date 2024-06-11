use camino::{Utf8Path, Utf8PathBuf};
use ignore::DirEntry;

use crate::{Error, FileMetadata, Result};

#[derive(Debug, Clone)]
pub struct LocalMetadata {
    relative_path: Utf8PathBuf,
    entry: DirEntry,
}

impl FileMetadata for LocalMetadata {
    fn path(&self) -> &Utf8Path {
        &self.relative_path
    }

    fn size(&self) -> Result<u64> {
        Ok(self.entry.metadata().map_err(|e| Error::Other(e.to_string()))?.len())
    }

    fn hash(&self) -> Result<String> {
        sha256::try_digest(self.entry.path()).map_err(|e| Error::Other(e.to_string()))
    }
}

impl LocalMetadata {
    pub fn new(relative_path: Utf8PathBuf, entry: DirEntry) -> Self {
        Self { relative_path, entry }
    }
}
