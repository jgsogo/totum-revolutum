enum SnapshotStatus {
    ToBeDeleted,
    New,
    Modified,
    Idle,
}

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: std::marker::Sync + std::marker::Send + std::fmt::Debug {
    fn path(&self) -> &str;
    fn size(&self) -> i64;
    fn hash(&self) -> u64;
}

pub struct BasePointDiffImpl<T: FileMetadata> {
    tx: tokio::sync::mpsc::UnboundedSender<T>,
}

impl<T: FileMetadata> BasePointDiffImpl<T> {
    pub fn new(tx: tokio::sync::mpsc::UnboundedSender<T>) -> BasePointDiffImpl<T> {
        BasePointDiffImpl::<T> { tx }
    }

    pub fn file_found(&self, metadata: T) {
        self.tx.send(metadata).unwrap();
    }
}
