pub const MAX_BUFFER: usize = 100;

#[derive(Debug)]
#[allow(dead_code)]
pub enum SnapshotStatus {
    ToBeDeleted,
    New,
    Modified,
    Idle,
}

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: std::marker::Sync + std::marker::Send + std::fmt::Debug + std::clone::Clone {
    /// Shared identifier for the file
    fn id(&self) -> &str;
    fn size(&self) -> u64;
    fn hash(&self) -> String;

    fn eq<T: FileMetadata>(&self, other: &T) -> bool {
        self.size() == other.size() && self.hash() == other.hash()
    }
}

pub trait BasePoint<T: FileMetadata> {}
