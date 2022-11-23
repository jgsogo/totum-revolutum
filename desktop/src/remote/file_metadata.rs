use crate::diff::basepoint::FileMetadata;

#[derive(Debug)]
pub struct RemoteMetadata {
    path: String,
}

impl FileMetadata for RemoteMetadata {
    fn size(&self) -> i64 {
        32
    }
    fn hash(&self) -> u64 {
        32
    }

    fn id(&self) -> &str {
        &self.path
    }
}
