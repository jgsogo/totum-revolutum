use crate::actions::{copy_file, move_file};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::sync::oneshot::Receiver;

use super::utils::normalize_path;
use super::{Error, Result};
use super::{File, FileMetadata};

/// Abstraction of a filesystem with methods to access its files
#[async_trait]
pub trait Filesystem: Send + Sync {
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
    /// Walk files in the filesystem, for each file found it will send it via `tx`. This belongs
    /// to the [`FilesystemRead`] because it **reads** the contents of the directories.
    async fn walk_directory(
        &self,
        tx: flume::Sender<Box<dyn FileMetadata>>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()>;

    /// Returns the [`FileMetadata`] for the given `path`
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>>;

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &Utf8Path) -> Result<bool>;

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>>;
}

/// Filesystem abstraction, only methods that require **write access**
#[async_trait]
pub trait FilesystemWrite: Send + Sync {
    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    /// This method returns an object implementing the [`File`] trait. Some implementations may
    /// return a [`Receiver`] that the caller can await for a couple of reasons:
    ///  * to receive any error that may happen from the file drop procedure
    ///  * to ensure that all the in-memory data is written to the file.
    ///
    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)>;

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

/// Filesystem abstraction, method that requires **read and write access**. Default implementation
/// relies on [`FilesystemRead::open`], [`FilesystemWrite::create`] and
/// [`FilesystemRemove::remove_file`], however, specific implementations can override it if there
/// is a more performant way to run these operations.
#[async_trait]
pub trait FilesystemInnerOperations: Filesystem + FilesystemRead + FilesystemWrite + FilesystemRemove {
    /// Copy
    async fn copy(&self, origin: &Utf8Path, target: &Utf8Path, force: bool) -> Result<Option<Receiver<Result<()>>>> {
        copy_file(self, self, origin, target, force).await
    }

    /// Rename
    async fn rename(&self, origin: &Utf8Path, target: &Utf8Path, force: bool) -> Result<Option<Receiver<Result<()>>>> {
        move_file(self, self, origin, target, force).await
    }
}
