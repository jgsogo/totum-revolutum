use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::utils::normalize_path;

use super::file_metadata::FileMetadata;
use super::File;

/// Abstract a filesystem, either local or remote and provide methods to access their files
#[async_trait]
pub trait Filesystem
where
    Self: Sync,
{
    type Metadata: FileMetadata;
    type File: File;

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
            Err(anyhow!(
                "Path '{}' is outside filesystem (root '{}')",
                path.display(),
                self.root().display()
            ))
        } else {
            Ok(path)
        }
    }

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(&self, tx: flume::Sender<Self::Metadata>, threads: usize) -> Result<()>;

    async fn create(&self, path: &Path) -> Result<Self::File>;

    /// Tries to open the file requested by the argument `path`. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Path) -> Result<Self::File>;
}
