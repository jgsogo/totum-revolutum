use super::DBFileMetadata;
use crate::database::{DBFile, Database};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use filesystem::{Error, File, Filesystem, Result};
use tracing::{error, info};

/// Implements [`filesystem::Filesystem`] for a _filesystem_ whose content is proxied in a
/// database. The database is the source of truth for the filesystem content, although the content
/// of the files is retrieved from the actual _filesystem_.
///
/// Note that the database and the storage may get out-of-sync if some files are added/removed to
/// the storage without updating the database (see [`super::DatabaseSync`]).
pub struct DatabaseBackup<TDatabase: Database, TFilesystem: Filesystem> {
    database: TDatabase,
    filesystem: TFilesystem,

    tx_file_close: flume::Sender<FileCloseMessage>,
    // thread_file_close: Option<JoinHandle<()>>,
}

pub(crate) enum FileCloseMessage {
    FileClosed(Utf8PathBuf),
    Stop,
}

impl<TDatabase: Database, TFilesystem: Filesystem> DatabaseBackup<TDatabase, TFilesystem> {
    pub fn new(database: TDatabase, filesystem: TFilesystem) -> Self {
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();

        // This async loop will take care of updating the DB after the file is closed.
        let db = database.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileClosed(path) => {
                        if let Err(e) = db.create_or_update_file(&path) {
                            error!("Error updating/creating entry in DB: {e}");
                        }
                    }
                    FileCloseMessage::Stop => {
                        info!("Received STOP message");
                        break;
                    }
                }
            }
        });

        Self {
            database,
            filesystem,
            tx_file_close: tx,
            //thread_file_close: Some(t),
        }
    }
}

impl<TDatabase: Database, TFilesystem: Filesystem> Drop for DatabaseBackup<TDatabase, TFilesystem> {
    fn drop(&mut self) {
        let _ = self.tx_file_close.send(FileCloseMessage::Stop);
    }
}

#[async_trait]
impl<TDatabase: Database, TFilesystem: Filesystem> Filesystem for DatabaseBackup<TDatabase, TFilesystem> {
    type Metadata = DBFileMetadata;

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(
        &self,
        tx: flume::Sender<Self::Metadata>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        for it in self.database.all_files()? {
            tx.send(it)
                .map_err(|e| Error::Other(format!("Error sending metadata: {e}")))?
        }
        Ok(())
    }

    /// Returns the [`Self::Metadata`] for the given `path`
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Self::Metadata> {
        self.database.get_file(path)
    }

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.database.file_exists(path)
    }

    /// Creates a file with this name in write-only mode. If it already exists, it will delete everything on it.
    async fn create(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let file = self.filesystem.create(path).await?;
        // TODO: Create entry in the database on only when it is closed??
        Ok(Box::new(DBFile {
            file,
            filepath: Some(path.to_path_buf()),
            tx: self.tx_file_close.clone(),
        }))
    }

    /// Tries to open the file requested by the argument `path` in read-only mode. Returns an object implementing
    /// a [`File`] or an error.
    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        // TODO: Check file is already in the database??
        let file = self.filesystem.open(path).await?;
        Ok(Box::new(DBFile {
            file,
            filepath: Some(path.to_path_buf()),
            tx: self.tx_file_close.clone(),
        }))
    }

    /// Creates the given directory and any intermediate one
    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.create_dir_all(path).await?;
        self.database.create_dir(path)
    }

    /// Copy
    async fn copy(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()> {
        self.filesystem.copy(origin, target).await?;
        self.database.copy(origin, target)
    }

    /// Rename
    async fn rename(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()> {
        self.filesystem.rename(origin, target).await?;
        self.database.rename(origin, target)
    }

    /// Removes a file from the filesystem.
    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_file(path).await?;
        self.database.remove_file(path)
    }

    /// Removes an empty directory.
    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_dir(path).await?;
        self.database.remove_dir(path)
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_dir_all(path).await?;
        self.database.remove_dir_all(path)
    }
}
