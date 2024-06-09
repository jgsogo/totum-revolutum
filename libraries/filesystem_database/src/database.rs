use crate::database_backup::FileCloseMessage;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use filesystem::{File, FileMetadata, Result};

pub trait Database: Sync + Send + Clone + 'static {
    // FIXME: This is basically the Filesystem trait... without the `open` and `create` methods. It´s
    // FIXME: much better to rely on the DB implementing the `Filesystme` trait and return a
    // FIXME: `NonUsableFile` from those methods. With this approach the mirror can be implemented
    // FIXME: between any two Filesystem implementors: something that runs the same operation on
    // FIXME: both (only need to generalize the `Drop` signal sending).
    // FIXME:
    // FIXME: Of course, if it´s just a mirror, it should be implemented in the `filesystem` crate.

    /// Returns an iterator through all the files
    fn all_files(&self) -> Result<impl Iterator<Item = DBFileMetadata>>;

    /// Returns a file given a path
    fn get_file(&self, path: &Utf8Path) -> Result<DBFileMetadata>;

    /// Returns if a file given a path exists
    fn file_exists(&self, path: &Utf8Path) -> Result<bool>;

    /// Creates or updates the file at the given path. It has to recompute all the information associated to the file
    fn create_or_update_file(&self, path: &Utf8Path) -> Result<DBFileMetadata>;

    /// Copies entry in the database
    /// // TODO: file?
    fn copy(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()>;

    /// Moves entry in the database
    /// /// // TODO: file?
    fn rename(&self, origin: &Utf8Path, target: &Utf8Path) -> Result<()>;

    fn remove_file(&self, path: &Utf8Path) -> Result<()>;

    /// Removes an empty directory
    fn remove_dir(&self, path: &Utf8Path) -> Result<()>;

    /// Removes an empty directory
    fn remove_dir_all(&self, path: &Utf8Path) -> Result<()>;

    /// Creates a directory (and any intermediate one)
    fn create_dir(&self, path: &Utf8Path) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct DBFileMetadata;

impl FileMetadata for DBFileMetadata {
    fn path(&self) -> &Utf8Path {
        todo!()
    }

    fn size(&self) -> Result<u64> {
        todo!()
    }

    fn hash(&self) -> Result<String> {
        todo!()
    }
}

pub(crate) struct DBFile {
    pub file: Box<dyn File>,
    pub filepath: Option<Utf8PathBuf>,
    pub tx: flume::Sender<FileCloseMessage>,
}

impl Drop for DBFile {
    fn drop(&mut self) {
        self.tx
            .send(FileCloseMessage::FileClosed(self.filepath.take().unwrap()))
            .unwrap()
    }
}

#[async_trait]
impl File for DBFile {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.file.read_to_end(buf).await
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.file.write_all(buf).await
    }

    async fn sync_all(&mut self) -> Result<()> {
        self.file.sync_all().await
    }
}
