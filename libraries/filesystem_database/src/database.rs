use crate::database_backup::FileCloseMessage;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use filesystem::{File, FileMetadata, Result};

// FIXME: Probably move closer to some database_utils
pub trait ObjectManager<Object, ID: ?Sized>: Sync {
    fn all(&self) -> Result<impl Iterator<Item = Object>>;

    fn get(&self, id: &ID) -> Result<Object>;

    fn exists(&self, id: &ID) -> Result<bool>;

    fn create(&self, obj: Object) -> Result<()>;
}

pub struct DBDirectory;

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
