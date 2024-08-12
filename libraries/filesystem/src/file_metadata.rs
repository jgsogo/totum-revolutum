use std::fmt::Debug;

use crate::{FilePath, FilePathBuf};

/// Allows access to file metadata. The information here is enough to compare two files and decide
/// if they are the same or not.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: FilePathBuf,
    pub hash: String,
    pub size: u64,
}

impl FileMetadata {
    /// Path inside the [`crate::Filesystem`]
    ///
    /// This path identifies one-to-one every file inside a filesystem (it's the relative path
    /// from the root of the filesystem).
    pub fn path(&self) -> &FilePath {
        &self.path
    }

    /// The size of the file
    pub fn size(&self) -> u64 {
        self.size
    }

    /// A hash (sha256) computed from the contents of the file
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl PartialEq for FileMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size() && self.hash() == other.hash()
    }
}
