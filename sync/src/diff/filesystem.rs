use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::utils::normalize_path;

use super::File;
use super::file_metadata::FileMetadata;

/// Represents the local or remote storage as a filesystem
#[async_trait]
pub trait Filesystem
where
    Self: Sync,
{
    type Metadata: FileMetadata;

    fn root(&self) -> &Path;

    /// Checks that the given path relies within the filesystem. Returns the absolute path or
    /// an error
    fn check_path(&self, path: &Path) -> Result<PathBuf> {
        let path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root().join(path)
        };
        let path = normalize_path(path);
        if !path.starts_with(self.root()) {
            Err(anyhow!("Path {} is outside filesystem", path.display()))
        } else {
            Ok(path)
        }
    }

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(&self, tx: flume::Sender<Self::Metadata>, threads: usize) -> Result<()>;

    /// Tries to open the file requested by the argument `path`. Returns an object implementing
    /// a [`File`] or an error.
    fn open(&self, path: &Path) -> Result<Box<dyn File>>;
}
