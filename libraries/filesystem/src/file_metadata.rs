/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata
where
    Self: Sync + Send + std::fmt::Debug + Clone + From<Self::DirEntry>,
{
    type DirEntry;

    /// Shared identifier for the file
    fn id(&self) -> &str;
    fn size(&self) -> u64;
    fn hash(&self) -> String;

    fn eq<T: FileMetadata>(&self, other: &T) -> bool {
        self.size() == other.size() && self.hash() == other.hash()
    }
}
