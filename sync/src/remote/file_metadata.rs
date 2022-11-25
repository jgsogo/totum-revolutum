use crate::diff::basepoint::FileMetadata;

#[derive(Debug, Clone)]
pub struct RemoteMetadata {
    path: String,
}

impl FileMetadata for RemoteMetadata {
    fn id(&self) -> &str {
        &self.path
    }
    fn size(&self) -> u64 {
        32
    }

    fn hash(&self) -> String {
        "32".into()
    }
}
