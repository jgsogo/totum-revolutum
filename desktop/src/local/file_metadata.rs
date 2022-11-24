use crate::diff::basepoint::FileMetadata;
use ignore::DirEntry;
use std::path::Path;

pub trait LocalFileMetadata: FileMetadata {
    fn from_direntry(base_path: &Path, entry: DirEntry) -> Self;

    fn path(&self) -> &Path;
}

#[derive(Debug, Clone)]
pub struct LocalMetadata {
    id: String,
    entry: DirEntry,
}

impl FileMetadata for LocalMetadata {
    fn size(&self) -> u64 {
        self.entry.metadata().unwrap().len()
    }
    fn hash(&self) -> String {
        sha256::try_digest(self.path()).unwrap()
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
            entry,
        }
    }

    fn path(&self) -> &Path {
        self.entry.path()
    }
}
