use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use tokio::sync::oneshot::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use crate::{Error, File, Filesystem, FilesystemRead, FilesystemRemove, FilesystemWrite};
use crate::{FileMetadata, Result};

type FileCloseMessageData = (
    Box<dyn File>,
    Option<Receiver<Result<()>>>,
    Utf8PathBuf,
    Sender<Result<()>>,
);

enum FileCloseMessage {
    FromReadOnlyFile(FileCloseMessageData),
    FromWriteFile(FileCloseMessageData),
    Stop,
}

/// A wrapper for a filesystem that implements [`FilesystemWrite`]. This wrapper will execute a
/// function after a file is dropped
pub struct AsyncFileDropImpl<T: FilesystemWrite> {
    filesystem: Arc<Mutex<Option<T>>>,

    tx_file_close: flume::Sender<FileCloseMessage>,
    thread_file_close: Option<JoinHandle<()>>,

    wrap_create: bool,
    wrap_open: bool,
}

impl<T: FilesystemWrite + 'static> AsyncFileDropImpl<T> {
    /// Args:
    ///  * `filesystem`: Filesystem to wrap. It needs to implement [`Filesystem`] and
    ///     [`FilesystemWrite`] traits. [`Filesystem::sync_all`] is used to close the thread and
    ///     [`FilesystemWrite::create`] is the method that we are actually wrapping here
    ///
    ///  * `func`: Function to execute on the file just before it's dropped.
    pub fn new<F, Fut>(filesystem: T, func_for_readonly_file: Option<F>, func_for_write_file: Option<F>) -> Self
    where
        F: Fn(Box<dyn File>, Option<Receiver<Result<()>>>, Arc<Mutex<Option<T>>>, Utf8PathBuf) -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let wrap_open = func_for_readonly_file.is_some();
        let wrap_create = func_for_write_file.is_some();

        let filesystem = Arc::new(Mutex::new(Some(filesystem)));
        let fs = filesystem.clone();
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();
        let t = tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FromReadOnlyFile((file, rx_filesystem, path, sender)) => {
                        let r = func_for_readonly_file.as_ref().unwrap()(file, rx_filesystem, fs.clone(), path).await;
                        if sender.send(r).is_err() {
                            debug!("Error sending file close result. Receiver might have been dropped (and it's fine)")
                        };
                    }
                    FileCloseMessage::FromWriteFile((file, rx_filesystem, path, sender)) => {
                        let r = func_for_write_file.as_ref().unwrap()(file, rx_filesystem, fs.clone(), path).await;
                        if sender.send(r).is_err() {
                            debug!("Error sending file close result. Receiver might have been dropped (and it's fine)")
                        };
                    }
                    FileCloseMessage::Stop => {
                        info!("Received STOP message");
                        break;
                    }
                }
            }
        });

        Self {
            filesystem,
            tx_file_close: tx,
            thread_file_close: Some(t),
            wrap_open,
            wrap_create,
        }
    }

    pub fn new_call_sync_all(filesystem: T) -> Self {
        let sync_all = |file: Box<dyn File>, rx_filesystem: Option<Receiver<Result<()>>>, _fs, _path| async move {
            file.sync_all().await?;
            drop(file);
            if let Some(r) = rx_filesystem {
                r.await
                    .map_err(|e| Error::Other(format!("Error receiving drop result from rx_filesystem: {e}")))??;
            }
            Ok(())
        };

        Self::new(filesystem, Some(sync_all), Some(sync_all))
    }
}

#[async_trait]
impl<T: Filesystem + FilesystemWrite> Filesystem for AsyncFileDropImpl<T> {
    async fn sync_all(mut self) -> Result<()> {
        if self.tx_file_close.send(FileCloseMessage::Stop).is_ok() {
            self.thread_file_close
                .take()
                .ok_or_else(|| Error::Other("Thread is already closed!".to_string()))?
                .await
                .map_err(|e| Error::Other(e.to_string()))?;
        }

        self.filesystem.lock().await.take().unwrap().sync_all().await
    }
}

#[async_trait]
impl<T: FilesystemWrite> FilesystemWrite for AsyncFileDropImpl<T> {
    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        if self.wrap_create {
            let (file, rx_filesystem) = self.filesystem.lock().await.as_ref().unwrap().create(path).await?;
            let (tx, rx) = tokio::sync::oneshot::channel();
            let ret = FileAsyncDrop {
                file: Some(file),
                rx_filesystem,
                path: Some(path.to_path_buf()),
                tx_filesystem_close: self.tx_file_close.clone(),
                tx_file_close: Some(tx),
                is_readonly: false,
            };
            Ok((Box::new(ret), Some(rx)))
        } else {
            self.filesystem.lock().await.as_ref().unwrap().create(path).await
        }
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem
            .lock()
            .await
            .as_ref()
            .unwrap()
            .create_dir_all(path)
            .await
    }
}

#[async_trait]
impl<T: FilesystemWrite + FilesystemRead> FilesystemRead for AsyncFileDropImpl<T> {
    async fn walk_directory(
        &self,
        tx: flume::Sender<Box<dyn FileMetadata>>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.filesystem
            .lock()
            .await
            .as_ref()
            .unwrap()
            .walk_directory(tx, threads, custom_ignore_filename)
            .await
    }
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        self.filesystem.lock().await.as_ref().unwrap().get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.filesystem.lock().await.as_ref().unwrap().exists(path).await
    }
    async fn open(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        if self.wrap_open {
            let (file, rx_filesystem) = self.filesystem.lock().await.as_ref().unwrap().open(path).await?;
            let (tx, rx) = tokio::sync::oneshot::channel();
            let ret = FileAsyncDrop {
                file: Some(file),
                rx_filesystem,
                path: Some(path.to_path_buf()),
                tx_filesystem_close: self.tx_file_close.clone(),
                tx_file_close: Some(tx),
                is_readonly: true,
            };
            Ok((Box::new(ret), Some(rx)))
        } else {
            self.filesystem.lock().await.as_ref().unwrap().open(path).await
        }
    }
}

#[async_trait]
impl<T: FilesystemWrite + FilesystemRemove> FilesystemRemove for AsyncFileDropImpl<T> {
    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.lock().await.as_ref().unwrap().remove_file(path).await
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem.lock().await.as_ref().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem
            .lock()
            .await
            .as_ref()
            .unwrap()
            .remove_dir_all(path)
            .await
    }
}

/// A wrapper over [`FileImpl`] that sends the message to [`AsyncFileDropImpl`] when it is dropped.
struct FileAsyncDrop {
    /// The object representing the file
    file: Option<Box<dyn File>>,

    /// The receiver returned together with the file from the filesystem
    rx_filesystem: Option<Receiver<Result<()>>>,

    /// Whether the file was opened in read-only mode or not.
    is_readonly: bool,

    /// Path to the file
    path: Option<Utf8PathBuf>,

    /// A sender to be called from drop. The message will arrive [`super::FilesystemLocal`] and it
    /// will take care of calling [`AsyncFile::sync_all`] to ensure that all data is written to
    /// the filesystem
    tx_filesystem_close: flume::Sender<FileCloseMessage>,

    /// A channel to notify the user that all drop operations have finished and it's safe to exit
    /// the application.
    tx_file_close: Option<Sender<Result<()>>>,
}

impl Drop for FileAsyncDrop {
    fn drop(&mut self) {
        let data = (
            self.file.take().unwrap(),
            self.rx_filesystem.take(),
            self.path.take().unwrap(),
            self.tx_file_close.take().unwrap(),
        );
        let msg = if self.is_readonly {
            FileCloseMessage::FromReadOnlyFile(data)
        } else {
            FileCloseMessage::FromWriteFile(data)
        };

        if let Err(e) = self.tx_filesystem_close.send(msg) {
            error!("Failed to send FileCloseMessage: {e}");
        }
    }
}

#[async_trait]
impl File for FileAsyncDrop {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        let file = self.file.as_mut().unwrap();
        file.read_to_end(buf).await
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let file = self.file.as_mut().unwrap();
        file.read(buf).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let file = self.file.as_mut().unwrap();
        file.write_all(buf).await
    }

    async fn sync_all(&self) -> Result<()> {
        let file = self.file.as_ref().unwrap();
        file.sync_all().await
    }
}
