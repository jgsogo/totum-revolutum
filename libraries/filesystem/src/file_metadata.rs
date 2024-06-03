use std::fmt::Debug;

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: Clone + Send + Sync + From<Self::DirEntry> + Debug {
    type DirEntry;

    /// Unique identifier for the file (inside the filesystem)
    fn id(&self) -> &str;

    /// The size of the file
    fn size(&self) -> u64;

    /// A hash computed from the contents of the file
    fn hash(&self) -> String;

    /// Equality at [`FileMetadata`] level: checks only size and hash
    fn eq<T: FileMetadata>(&self, other: &T) -> bool {
        self.size() == other.size() && self.hash() == other.hash()
    }
}
