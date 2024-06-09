use super::DBFileMetadata;
use crate::database::{DBDirectory, DBFile, ObjectManager};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use filesystem::{Error, File, Filesystem, Result};
use tracing::info;

/// Implements [`filesystem::Filesystem`] for a _filesystem_ whose content is proxied in a
/// database. The database is the source of truth for the filesystem content, although the content
/// of the files is retrieved from the actual _filesystem_.
///
/// Note that the database and the storage may get out-of-sync if some files are added/removed to
/// the storage without updating the database (see [`super::DatabaseSync`]).
pub struct DatabaseBackup<
    DbFiles: ObjectManager<DBFileMetadata, Utf8Path>,
    DbDirs: ObjectManager<DBDirectory, Utf8Path>,
    FLS: Filesystem,
> {
    db_files: DbFiles,
    db_directories: DbDirs,
    filesystem: FLS,

    tx_file_close: flume::Sender<FileCloseMessage>,
    //vthread_file_close: Option<JoinHandle<()>>,
}

pub(crate) enum FileCloseMessage {
    FileClosed(Utf8PathBuf),
    Stop,
}

impl<
        DbFiles: ObjectManager<DBFileMetadata, Utf8Path>,
        DbDirs: ObjectManager<DBDirectory, Utf8Path>,
        FLS: Filesystem,
    > DatabaseBackup<DbFiles, DbDirs, FLS>
{
    pub fn new(db_files: DbFiles, db_directories: DbDirs, filesystem: FLS) -> Self {
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();

        // This async loop will take care of calling the 'file_close' method when RemoteFiles go out
        //  of scope. Here we can call this async method, while it is not possible to do it in the
        //  `Drop` implementation of those files.
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileClosed(_path) => {
                        // TODO: Update entry in the database (or sometimes create)
                    }
                    FileCloseMessage::Stop => {
                        info!("Received STOP message");
                        break;
                    }
                }
            }
        });

        Self {
            db_files,
            db_directories,
            filesystem,
            tx_file_close: tx,
            //thread_file_close: Some(t),
        }
    }
}

impl<
        DbFiles: ObjectManager<DBFileMetadata, Utf8Path>,
        DbDirs: ObjectManager<DBDirectory, Utf8Path>,
        FLS: Filesystem,
    > Drop for DatabaseBackup<DbFiles, DbDirs, FLS>
{
    fn drop(&mut self) {
        let _ = self.tx_file_close.send(FileCloseMessage::Stop);
    }
}

#[async_trait]
impl<
        DbFiles: ObjectManager<DBFileMetadata, Utf8Path>,
        DbDirs: ObjectManager<DBDirectory, Utf8Path>,
        FLS: Filesystem,
    > Filesystem for DatabaseBackup<DbFiles, DbDirs, FLS>
{
    type Metadata = DBFileMetadata;

    /// Walk files in the filesystem, for each file found it will send it via `tx`
    async fn walk_directory(
        &self,
        tx: flume::Sender<Self::Metadata>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        for it in self.db_files.all()? {
            tx.send(it)
                .map_err(|e| Error::Other(format!("Error sending metadata: {e}")))?
        }
        Ok(())
    }

    /// Returns the [`Self::Metadata`] for the given `path`
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Self::Metadata> {
        self.db_files.get(path)
    }

    /// Returns true if the path points at an existing entity.
    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.db_files.exists(path)
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
        // TODO: Check file is already in the database
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
        self.db_directories.create(DBDirectory)
    }

    /// Copy
    async fn copy(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()> {
        self.filesystem.copy(origin, target).await?;
        todo!("Duplicate entry in the database (what if it points to a folder?)")
    }

    /// Rename
    async fn rename(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()> {
        self.filesystem.rename(origin, target).await?;
        todo!("Rename entry in the database (what if it points to a folder?)")
    }

    /// Removes a file from the filesystem.
    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_file(path).await?;
        todo!("Remove file entry from database")
    }

    /// Removes an empty directory.
    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_dir(path).await?;
        todo!("Remove directory (easy, it´s empty... but check)")
    }

    /// Removes a directory at this path, after removing all its contents. Use carefully!
    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.remove_dir_all(path).await?;
        todo!("Remove directory and all the files into it (an subfolders)")
    }
}
