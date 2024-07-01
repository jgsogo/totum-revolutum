use async_trait::async_trait;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;
use tracing::trace;

use crate::actions::copy;

use super::{DirectoryPath, Error, FilePath, Result};
use super::{File, FileMetadata};

/// Abstraction of a filesystem with methods to access its files
#[async_trait]
pub trait Filesystem: Send + Sync {
    /// Waits for any pending operation and finishes this filesystem.
    async fn sync_all(self) -> Result<()>;

    /// Walk files in the filesystem, for each file found it will send it via `tx`. This belongs
    /// to the [`Filesystem`] because it **reads** the contents of the directories.
    ///
    /// TODO: Probably we need the possibility to choose the starting path
    async fn walk_directory(&self, tx: flume::Sender<FileMetadata>, ignore_filter: IgnoreFilter) -> Result<()>;

    /// Returns the [`FileMetadata`] for the given `path`. This operation blocks until the data is
    /// available (some filesystem implementations might not have this data available right at
    /// the moment a new file is created).
    async fn get_metadata(&self, path: &FilePath) -> Result<FileMetadata>;

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &FilePath) -> Result<bool>;

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an
    /// object implementing a [`File`] or an error. Some implementations may return a [`Receiver`]
    /// that the caller can await to receive any error that may happen from the file drop procedure.
    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)>;

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    /// This method returns an object implementing the [`File`] trait. Some implementations may
    /// return a [`Receiver`] that the caller can await for a couple of reasons:
    ///  * to receive any error that may happen from the file drop procedure
    ///  * to ensure that all the in-memory data is written to the file.
    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)>;

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()>;

    /// Removes a file from the filesystem.
    async fn remove_file(&mut self, path: &FilePath) -> Result<()>;

    /// Removes an empty directory.
    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()>;

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()>;

    /// Copies a file inside this same [`Filesystem`] from `origin` to `target` path.
    ///
    /// This method is default-implemented using [`Filesystem::open`] and [`Filesystem::create`], it
    /// should be overridden by [`Filesystem`] implementations that can optimize this copy.
    async fn internal_copy(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
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
        origin: &FilePath,
        target: &FilePath,
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
    ///
    /// [`Error:TargetFileExists`]: Error#variant.TargetFileExists
    async fn copy_from(
        &mut self,
        target: &FilePath,
        origin: &dyn Filesystem,
        origin_path: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        trace!("Copy from {} to {}", origin_path, target);
        if self.is_same(origin) {
            trace!("Origin and target filesystems are the same");
            self.internal_copy(target, origin_path, force).await
        } else {
            if !force && self.exists(target).await? {
                trace!("Target already exists. Skip operation");
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
    ///
    /// [`Error:TargetFileExists`]: Error#variant.TargetFileExists
    async fn move_from(
        &mut self,
        target: &FilePath,
        origin: &mut dyn Filesystem,
        origin_path: &FilePath,
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
