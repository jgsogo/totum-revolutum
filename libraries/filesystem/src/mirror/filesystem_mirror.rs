use crate::actions::copy_file;
use crate::{Error, File, Filesystem, Result};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::task::JoinHandle;
use tracing::{error, info};

enum FileCloseMessage {
    FileCloseMessage(Utf8PathBuf),
    Stop,
}

/// Applies the same changes to two [`Filesystem`] implementations. Changes are applied on the first
/// one and then mirrored to the second.
///
/// Note.- It´s up to the user to ensure that both [`Filesystem`] instances contain the same files
/// (or the operations running on them, only touch files that are present on both).
pub struct FilesystemMirror<TFilesystem1: Filesystem, TFilesystem2: Filesystem> {
    filesystem1: TFilesystem1,
    filesystem2: TFilesystem2,

    tx_file_close: Sender<FileCloseMessage>,
    thread_file_close: Option<JoinHandle<()>>,
}

struct FileCloned {
    file: Box<dyn File>,
    path: Utf8PathBuf,
    tx_file_close: Sender<FileCloseMessage>,
}

impl Drop for FileCloned {
    fn drop(&mut self) {
        let msg = FileCloseMessage::FileCloseMessage(self.path.clone());
        if let Err(e) = self.tx_file_close.send(msg) {
            error!("Error sending sync message to filesystem2: {e}")
        }
    }
}

#[async_trait]
impl File for FileCloned {
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

pub trait FilesystemMirrorTrait: Filesystem + Send + Clone + 'static {}

impl<TFilesystem1: FilesystemMirrorTrait, TFilesystem2: FilesystemMirrorTrait>
    FilesystemMirror<TFilesystem1, TFilesystem2>
{
    pub fn new(filesystem1: TFilesystem1, filesystem2: TFilesystem2) -> Self {
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();

        let fs1 = filesystem1.clone();
        let fs2 = filesystem2.clone();
        let t = tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileCloseMessage(path) => {
                        // Copy/override file to the second filesystem
                        if let Err(e) = copy_file(&fs1, &fs2, &path, &path, true).await {
                            error!("Error copying the file to the other filesystem: {e}")
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
            filesystem1,
            filesystem2,
            tx_file_close: tx,
            thread_file_close: Some(t),
        }
    }

    /// Complete any pending operations: joins the file_close thread
    pub async fn flush(&mut self) -> Result<()> {
        if self.tx_file_close.send(FileCloseMessage::Stop).is_ok() {
            self.thread_file_close
                .take()
                .ok_or_else(|| Error::Other("Thread is already closed!".to_string()))?
                .await
                .map_err(|e| Error::Other(e.to_string()))?;
        }
        Ok(())
    }
}

impl<TFilesystem1: Filesystem, TFilesystem2: Filesystem> Drop for FilesystemMirror<TFilesystem1, TFilesystem2> {
    fn drop(&mut self) {
        // Send the close signal... in case it was not already closed
        let _ = self.tx_file_close.send(FileCloseMessage::Stop);
    }
}

#[async_trait]
impl<TFilesystem1: Filesystem, TFilesystem2: Filesystem> Filesystem for FilesystemMirror<TFilesystem1, TFilesystem2> {
    type Metadata = TFilesystem1::Metadata;

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.filesystem1
            .walk_directory(tx, threads, custom_ignore_filename)
            .await
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Self::Metadata> {
        self.filesystem1.get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.filesystem1.exists(path).await
    }

    async fn create(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let file = self.filesystem1.create(path).await?;
        Ok(Box::new(FileCloned {
            file,
            path: path.to_path_buf(),
            tx_file_close: self.tx_file_close.clone(),
        }))
    }

    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let file = self.filesystem1.open(path).await?;
        Ok(Box::new(FileCloned {
            file,
            path: path.to_path_buf(),
            tx_file_close: self.tx_file_close.clone(),
        }))
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.create_dir_all(path).await?;
        self.filesystem2.create_dir_all(path).await
    }

    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.remove_file(path).await?;
        self.filesystem2.remove_file(path).await
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.remove_dir(path).await?;
        self.filesystem2.remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.remove_dir_all(path).await?;
        self.filesystem2.remove_dir_all(path).await
    }
}
