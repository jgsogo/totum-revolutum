use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use pcloud_sdk::utils::normalize_path;

use crate::filesystem::{File, FileMetadata};

/// Abstract a filesystem, either local or remote and provide methods to access their files
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
            Err(anyhow!(
                "Path '{}' is outside filesystem (root '{}')",
                path.display(),
                self.root().display()
            ))
        } else {
            Ok(path)
        }
    }

    /// Returns the relative path for any given one
    fn rel_path(&self, path: &Path) -> Result<PathBuf> {
        let abs_path = self.check_path(path)?;
        let r = abs_path.strip_prefix(self.root()).map_err(|e| anyhow!(e));
        r.map(|p| p.to_path_buf())
    }

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(&self, tx: flume::Sender<Self::Metadata>, threads: usize) -> Result<()>;

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    async fn create(&self, path: &Path) -> Result<Box<dyn File>>;

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Path) -> Result<Box<dyn File>>;

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&self, path: &Path) -> Result<()>;

    /// Copy
    async fn copy(&self, _origin: &Path, _target: &Path) -> Result<()> {
        todo!("A default `copy` using existing methods is not implemented")
    }

    /// Rename
    async fn rename(&self, _origin: &Path, _target: &Path) -> Result<()> {
        todo!("A default `rename` using existing methods is not implemented")
    }

    /// Removes a file from the filesystem.
    async fn remove_file(&self, path: &Path) -> Result<()>;

    /// Removes an empty directory.
    async fn remove_dir(&self, path: &Path) -> Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, path: &Path) -> Result<()>;
}
