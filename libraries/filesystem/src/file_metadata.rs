use crate::Result;
use camino::Utf8Path;
use std::fmt::Debug;

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: Send + Sync + Debug {
    /// Path inside the [`crate::Filesystem`]
    ///
    /// This path identifies one-to-one every file inside a filesystem. It should be the
    /// **relative path** starting from the root of the filesystem.
    fn path(&self) -> &Utf8Path;

    /// The size of the file
    /// FIXME: Remove the Result. It can return the u64 directly
    fn size(&self) -> Result<u64>;

    /// A hash computed from the contents of the file
    /// FIXME: Remove the Result. It can return the String directly (maybe &str)
    fn hash(&self) -> Result<String>;

    /// Equality at [`FileMetadata`] level: checks only size and hash
    fn eq(&self, other: &dyn FileMetadata) -> Result<bool> {
        Ok(self.size()? == other.size()? && self.hash()? == other.hash()?)
    }
}
