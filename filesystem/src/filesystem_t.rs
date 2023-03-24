use anyhow::Result;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};

use super::{File, FileMetadata};

/// Abstract a filesystem, either local or remote and provide methods to access its files
#[async_trait]
pub trait Filesystem
where
    Self: Sync,
{
    /// Information about a File in this filesystem
    type Metadata: FileMetadata;

    /// A path-like type. It guarantees that the inner path is contained inside this filesystem abstraction
    type FilesystemPath;

    /// Takes a naïve `path` and converts it into a [`FilesystemPath`]. The input `path` needs to
    /// be contained within the filesystem
    fn to_filesystem_path(&self, path: &Utf8Path) -> Result<Self::FilesystemPath>;

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(
        &self,
        tx: flume::Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()>;

    /// Returns true if the path points to an existing entity.
    async fn exists(&self, path: &Utf8Path) -> Result<bool>;

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    async fn create(&self, path: &Utf8Path) -> Result<Box<dyn File>>;

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>>;

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()>;

    /// Copy
    async fn copy(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!("A default `copy` using existing methods is not implemented")
    }

    /// Rename
    async fn rename(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!("A default `rename` using existing methods is not implemented")
    }

    /// Removes a file from the filesystem.
    async fn remove_file(&self, path: &Utf8Path) -> Result<()>;

    /// Removes an empty directory.
    async fn remove_dir(&self, path: &Utf8Path) -> Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()>;
}
