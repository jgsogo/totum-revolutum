use camino::Utf8Path;

use crate::{Error, FileMetadata, FilePath, FilePathBuf, Result};

#[derive(Debug, Clone)]
pub struct LocalMetadata {
    path: FilePathBuf,
    size: u64,
    hash: String,
}

impl FileMetadata for LocalMetadata {
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

impl LocalMetadata {
    pub fn from_filesystem(abs_path: &Utf8Path, path: &FilePath) -> Result<Self> {
        let size = abs_path.metadata().map_err(Error::IoError)?.len();
        let hash = sha256::try_digest(abs_path)
            .map_err(|e| Error::Other(format!("Cannot compute sha256 of given file: {}", e)))?;
        Ok(LocalMetadata {
            path: path.to_filepath_buf(),
            size,
            hash,
        })
    }
}
