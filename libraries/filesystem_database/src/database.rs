use camino::Utf8Path;
use filesystem::{FileMetadata, Result};

pub trait Database: Sync {}

#[derive(Debug, Clone)]
pub struct DBFileMetadata;

impl FileMetadata for DBFileMetadata {
    fn path(&self) -> &Utf8Path {
        todo!()
    }

    fn size(&self) -> Result<u64> {
        todo!()
    }

    fn hash(&self) -> Result<String> {
        todo!()
    }
}
