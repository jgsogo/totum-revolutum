use camino::Utf8Path;
use filesystem::{FileMetadata, Result};

#[derive(Clone, Debug)]
pub struct DBDieselFileMetadata;

impl FileMetadata for DBDieselFileMetadata {
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
