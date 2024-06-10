use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::sync::oneshot::Receiver;

use super::utils::normalize_path;
use super::{Error, Result};
use super::{File, FileMetadata};

/// Abstraction of a filesystem with methods to access its files
#[async_trait]
pub trait Filesystem {
    /// Normalizes the given `path` ensuring that it is a relative path that doesn't go outside
    /// its root folder. Returns the normalized version of that path
    fn check_path(&self, path: &Utf8Path) -> Result<Utf8PathBuf> {
        let path = normalize_path(path);
        if path.starts_with("../") {
            Err(Error::PathOutsideFilesystem)
        } else {
            Ok(path)
        }
    }

    /// Waits for any pending operation and finishes this filesystem.
    async fn sync_all(self) -> Result<()>;
}

/// Filesystem abstraction, only method that require READ access
#[async_trait]
pub trait FilesystemRead {
    type Metadata: FileMetadata; // TODO: Associated type or just return `Box<dyn FileMetadata>`?

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(
        &self,
        tx: flume::Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()>;

    /// Returns the [`Self::Metadata`] for the given `path`
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Self::Metadata>;

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &Utf8Path) -> Result<bool>;

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>>;
}

/// Filesystem abstraction, only method that require **write access**
#[async_trait]
pub trait FilesystemWrite {
    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    /// This method returns the [`File`] object and a [`Receiver`]. This
    /// receiver will be called after the file is dropped and any pending task is run by the
    /// underlying filesystem (some [`Filesystem`] implementations may run async functions after
    /// the file is dropped).
    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Receiver<Result<()>>)>;

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()>;
}

/// Filesystem abstraction, only methods that require **remove access**
#[async_trait]
pub trait FilesystemRemove {
    /// Removes a file from the filesystem.
    async fn remove_file(&self, path: &Utf8Path) -> Result<()>;

    /// Removes an empty directory.
    async fn remove_dir(&self, path: &Utf8Path) -> Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()>;
}

/// Filesystem abstraction, method that requires **read and write access**
#[async_trait]
pub trait FilesystemReadAndWrite: FilesystemRead + FilesystemWrite {
    /// Copy
    async fn copy(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!("A default `copy` using existing methods is not implemented")
    }

    /// Rename
    async fn rename(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!("A default `rename` using existing methods is not implemented")
    }
}
