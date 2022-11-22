use super::basepoint::FileMetadata;
use chrono::Local;
use ignore::DirEntry;
use std::convert::From;
use std::path::PathBuf;

pub struct LocalMetadata {
    path: PathBuf,
}

impl FileMetadata for LocalMetadata {
    fn size(&self) -> i64 {
        32
    }
    fn hash(&self) -> u64 {
        32
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }
}

impl From<DirEntry> for LocalMetadata {
    fn from(d: DirEntry) -> Self {
        LocalMetadata {
            path: d.path().to_path_buf(),
        }
    }
}
