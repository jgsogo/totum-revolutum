use async_trait::async_trait;

enum SnapshotStatus {
    ToBeDeleted,
    New,
    Modified,
    Idle,
}

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: std::marker::Sync {
    fn path(&self) -> &str;
    fn size(&self) -> i64;
    fn hash(&self) -> u64;
}

/*
pub struct FileMetadataImpl;

impl FileMetadata for FileMetadataImpl {
    fn size(&self) -> i64 {
        0
    }
    fn hash(&self) -> u64 {
        0
    }

    fn path(&self) -> &str {
        todo!()
    }
}
*/

pub struct BasePointDiffImpl<T: FileMetadata> {
    tx: tokio::sync::mpsc::UnboundedSender<T>,
}

impl<T: FileMetadata + std::marker::Send> BasePointDiffImpl<T> {
    pub fn new(tx: tokio::sync::mpsc::UnboundedSender<T>) -> BasePointDiffImpl<T> {
        BasePointDiffImpl::<T> { tx }
    }

    pub fn file_found(&self, metadata: T) {
        self.tx.send(metadata);
    }
}

/*
pub trait BasePointDiff: std::marker::Sync {
    /// File found on lhs side of the diff
    fn file_found(&self, metadata: impl FileMetadata);
}

impl<T: FileMetadata + std::marker::Send> BasePointDiff for BasePointDiffImpl<T> {
    fn file_found(&self, metadata: impl FileMetadata) {
        self.tx.send(metadata);
    }
}
*/
