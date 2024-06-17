use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::sync::oneshot::Receiver;

use utils::filesystem::normalize_path;

use crate::actions::copy;

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

    /// Walk files in the filesystem, for each file found it will send it via `tx`. This belongs
    /// to the [`Filesystem`] because it **reads** the contents of the directories.
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

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an
    /// object implementing a [`File`] or an error. Some implementations may return a [`Receiver`]
    /// that the caller can await to receive any error that may happen from the file drop procedure.
    async fn open(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)>;

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    /// This method returns an object implementing the [`File`] trait. Some implementations may
    /// return a [`Receiver`] that the caller can await for a couple of reasons:
    ///  * to receive any error that may happen from the file drop procedure
    ///  * to ensure that all the in-memory data is written to the file.
    async fn create(&mut self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)>;

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()>;

    /// Removes a file from the filesystem.
    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()>;

    /// Removes an empty directory.
    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()>;

    /// Copies a file inside this same [`Filesystem`] from `origin` to `target` path.
    ///
    /// This method is default-implemented using [`Filesystem::open`] and [`Filesystem::create`], it
    /// should be overridden by [`Filesystem`] implementations that can optimize this copy.
    async fn internal_copy(
        &mut self,
        origin: &Utf8Path,
        target: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if !force && self.exists(target).await? {
            return Err(Error::TargetFileExists);
        }

        let (mut origin_file, _) = self.open(origin).await?;
        let (mut target_file, rx) = self.create(target).await?;
        copy(&mut origin_file, &mut target_file).await?;
        Ok(rx)
    }

    /// Moves a file inside this same [`Filesystem`] from `origin` to `target` path.
    ///
    /// This method is default-implemented using [`Filesystem::open`], [`Filesystem::create`] and
    /// [`Filesystem::remove_file`]. It should be overridden by [`Filesystem`] implementations
    /// that can optimize this copy and remove.
    async fn internal_move(
        &mut self,
        origin: &Utf8Path,
        target: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if !force && self.exists(target).await? {
            return Err(Error::TargetFileExists);
        }

        let r = self.internal_copy(origin, target, force).await?;
        self.remove_file(origin).await?;
        Ok(r)
    }
}

/// Declares operations in a filesystem that involve other filesystems
#[async_trait]
pub trait FilesystemOps: Filesystem + Sized {
    fn is_same(&self, other: &dyn Filesystem) -> bool {
        let lhs: *const dyn Filesystem = self;
        let rhs: *const dyn Filesystem = other;
        std::ptr::addr_eq(lhs, rhs)
    }

    /// Copies a file from `origin` [`Filesystem`] into `self` [`Filesystem`]. The flag `force` indicates if the
    /// target file should be overridden or not in case it already exists (raises [`Error:TargetFileExists`]).
    async fn copy_from(
        &mut self,
        target: &Utf8Path,
        origin: &dyn Filesystem,
        origin_path: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if self.is_same(origin) {
            self.internal_copy(target, origin_path, force).await
        } else {
            if !force && self.exists(target).await? {
                return Err(Error::TargetFileExists);
            }

            let (mut origin_file, _) = origin.open(origin_path).await?;
            let (mut target_file, rx) = self.create(target).await?;
            copy(&mut origin_file, &mut target_file).await?;
            Ok(rx)
        }
    }

    /// Moves a file from `origin` [`Filesystem`] into `self` [`Filesystem`]. The flag `force` indicates if the
    /// target file should be overridden or not in case it already exists (raises [`Error:TargetFileExists`]).
    async fn move_from(
        &mut self,
        target: &Utf8Path,
        origin: &mut dyn Filesystem,
        origin_path: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if self.is_same(origin) {
            self.internal_move(target, origin_path, force).await
        } else {
            if !force && self.exists(target).await? {
                return Err(Error::TargetFileExists);
            }

            let (mut origin_file, _) = origin.open(origin_path).await?;
            let (mut target_file, rx) = self.create(target).await?;
            copy(&mut origin_file, &mut target_file).await?;
            origin.remove_file(origin_path).await?;
            Ok(rx)
        }
    }
}

impl<T: Filesystem + Sized> FilesystemOps for T {}
