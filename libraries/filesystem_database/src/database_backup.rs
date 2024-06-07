use super::{DBFileMetadata, Database};
use async_trait::async_trait;
use camino::Utf8Path;
use filesystem::{File, Filesystem, Result};

/// Implements [`filesystem::Filesystem`] for a _filesystem_ whose content is proxied in a
/// database. The database is the source of truth for the filesystem content, although the content
/// of the files is retrieved from the actual _filesystem_.
///
/// Note that the database and the storage may get out-of-sync if some files are added/removed to
/// the storage without updating the database (see [`super::DatabaseSync`]).
pub struct DatabaseBackup<DB: Database, FLS: Filesystem> {
    _database: DB,
    _filesystem: FLS,
}

#[async_trait]
impl<DB: Database, FLS: Filesystem> Filesystem for DatabaseBackup<DB, FLS> {
    type Metadata = DBFileMetadata;

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(
        &self,
        _tx: flume::Sender<Self::Metadata>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        todo!()
    }

    /// Returns the [`Self::Metadata`] for the given `path`
    async fn get_metadata(&self, _path: &Utf8Path) -> Result<Self::Metadata> {
        todo!()
    }

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, _path: &Utf8Path) -> Result<bool> {
        todo!()
    }

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    async fn create(&self, _path: &Utf8Path) -> Result<Box<dyn File>> {
        todo!()
    }

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, _path: &Utf8Path) -> Result<Box<dyn File>> {
        todo!()
    }

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }

    /// Copy
    async fn copy(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!()
    }

    /// Rename
    async fn rename(&self, _origin: &Utf8Path, _target: &Utf8Path) -> Result<()> {
        todo!()
    }

    /// Removes a file from the filesystem.
    async fn remove_file(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }

    /// Removes an empty directory.
    async fn remove_dir(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }
}
