use crate::actions::basepoint::FileMetadata;
use ignore::DirEntry;
use std::path::{Path, PathBuf};

pub trait LocalFileMetadata: FileMetadata {
    fn from_direntry(base_path: &Path, entry: DirEntry) -> Self;

    fn path(&self) -> &Path;
}

#[derive(Debug)]
pub struct LocalMetadata {
    id: String,
    path: PathBuf,
}

impl FileMetadata for LocalMetadata {
    fn size(&self) -> i64 {
        32
    }
    fn hash(&self) -> u64 {
        32
    }

    fn id(&self) -> &str {
        &self.id
    }
}

impl LocalFileMetadata for LocalMetadata {
    fn from_direntry(base_path: &Path, entry: DirEntry) -> Self {
        Self {
            id: entry
                .path()
                .strip_prefix(base_path)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            path: entry.path().to_path_buf(),
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}
