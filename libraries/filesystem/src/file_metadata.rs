use crate::{FilePath, Result};
use std::fmt::Debug;

/// Allows access to file metadata. This is useful in case the information
/// is not already available or it is preferred to compute it on-demand (computing
/// hash can take some time)
pub trait FileMetadata: Send + Sync + Debug {
    /// Path inside the [`crate::Filesystem`]
    ///
    /// This path identifies one-to-one every file inside a filesystem (it's the relative path
    /// from the root of the filesystem).
    fn path(&self) -> &FilePath;

    /// The size of the file
    fn size(&self) -> u64;

    /// A hash (sha256) computed from the contents of the file
    fn hash(&self) -> &str;

    /// Equality at [`FileMetadata`] level: checks only size and hash
    fn eq(&self, other: &dyn FileMetadata) -> Result<bool> {
        Ok(self.size() == other.size() && self.hash() == other.hash())
    }
}
