use super::basepoint::FileMetadata;

#[derive(Debug)]
struct RemoteMetadata {
    path: String,
}

impl FileMetadata for RemoteMetadata {
    fn size(&self) -> i64 {
        32
    }
    fn hash(&self) -> u64 {
        32
    }

    fn path(&self) -> &str {
        &self.path
    }
}
