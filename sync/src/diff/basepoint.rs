pub const MAX_BUFFER: usize = 100;

#[derive(Debug)]
pub enum SnapshotStatus {
    ToBeDeleted,
    New,
    Modified,
    Idle,
}

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata:
    std::marker::Sync + std::marker::Send + std::fmt::Debug + std::clone::Clone
{
    /// Shared identifier for the file
    fn id(&self) -> &str;
    fn size(&self) -> u64;
    fn hash(&self) -> String;

    fn eq<T: FileMetadata>(&self, other: &T) -> bool {
        self.size() == other.size() && self.hash() == other.hash()
    }
}

/// Deal with the [`FileMetadata`] that is being gathered and sends it to the
/// differ connected to it.
#[derive(Clone)]
pub struct BasePointDiffImpl<T: FileMetadata>
where
    T: Clone,
{
    tx: flume::Sender<T>,
}

impl<T: FileMetadata> BasePointDiffImpl<T> {
    pub fn new(tx: flume::Sender<T>) -> BasePointDiffImpl<T> {
        BasePointDiffImpl::<T> { tx }
    }

    pub fn file_found(&self, metadata: T) {
        self.tx.send(metadata).unwrap();
    }
}
