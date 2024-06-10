use std::future::Future;

use async_trait::async_trait;
use flume::Sender;
use tokio::sync::oneshot::Receiver;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use crate::{Error, File, Result};

enum FileCloseMessage<FileImpl: File> {
    FileCloseMessage((FileImpl, tokio::sync::oneshot::Sender<Result<()>>)),
    Stop,
}

#[derive(Debug)]
pub struct FilesystemAsyncDrop<FileImpl: File + 'static> {
    tx_file_close: Sender<FileCloseMessage<FileImpl>>,
    thread_file_close: Option<JoinHandle<()>>,
}

impl<FileImpl: File + 'static> FilesystemAsyncDrop<FileImpl> {
    pub fn new<F, Fut>(func: F) -> Self
    where
        F: Fn(FileImpl) -> Fut + Send + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        // Spawns a thread that will execute async operation called from File drop
        let (tx, rx) = flume::unbounded::<FileCloseMessage<FileImpl>>();
        let t = tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileCloseMessage((file_impl, sender)) => {
                        let r = func(file_impl).await;
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
            tx_file_close: tx,
            thread_file_close: Some(t),
        }
    }

    /// Send [`FileCloseMessage::Stop`] message to the thread and waits for it to join
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

    pub fn file_wrapped(&self, file: FileImpl) -> (Box<dyn File>, Receiver<Result<()>>) {
        let (file, rx) = FileAsyncDrop::new(file, self.tx_file_close.clone());
        (Box::new(file), rx)
    }
}
//
// #[async_trait]
// pub trait FilesystemAsyncDrop<FileImpl: File + 'static> {
//     fn start_thread_file_close<F, Fut>(func: F) -> (Sender<FileCloseMessage<FileImpl>>, JoinHandle<()>)
//         where
//             F: Fn(FileImpl) -> Fut + Send + 'static,
//             Fut: Future<Output=Result<()>> + Send + 'static,
//     {
//         // Spawns a thread that will execute async operation called from File drop
//         let (tx, rx) = flume::unbounded::<FileCloseMessage<FileImpl>>();
//         let t = tokio::spawn(async move {
//             while let Ok(msg) = rx.recv_async().await {
//                 match msg {
//                     FileCloseMessage::FileCloseMessage((file_impl, sender)) => {
//                         let r = func(file_impl).await;
//                         if let Err(_) = sender.send(r) {
//                             debug!("Error sending file close result. Receiver might have been dropped (and it's fine)")
//                         };
//                     }
//                     FileCloseMessage::Stop => {
//                         info!("Received STOP message");
//                         break;
//                     }
//                 }
//             }
//         });
//         (tx, t)
//     }
//
//     fn get_tx_file_close(&self) -> &Sender<FileCloseMessage<FileImpl>>;
//
//     fn take_thread_file_close(&mut self) -> Result<JoinHandle<()>>;
//
//     /// Send [`FileCloseMessage::Stop`] message to the thread and waits for it to join
//     async fn flush(&mut self) -> Result<()> {
//         let tx_file_close = self.get_tx_file_close();
//         if tx_file_close.send(FileCloseMessage::Stop).is_ok() {
//             let thread = self.take_thread_file_close()?;
//             thread.await.map_err(|e| Error::Other(e.to_string()))?;
//         }
//         Ok(())
//     }
// }

struct FileAsyncDrop<FileImpl: File> {
    /// The object representing the file
    file: Option<FileImpl>,

    /// A sender to be called from drop. The message will arrive [`super::FilesystemLocal`] and it
    /// will take care of calling [`AsyncFile::sync_all`] to ensure that all data is written to
    /// the filesystem
    tx_filesystem_close: flume::Sender<FileCloseMessage<FileImpl>>,

    /// A channel to notify the user that all drop operations have finished and it's safe to exit
    /// the application.
    tx_file_close: Option<tokio::sync::oneshot::Sender<Result<()>>>,
}

impl<FileImpl: File> FileAsyncDrop<FileImpl> {
    fn new(
        file: FileImpl,
        tx_filesystem_close: flume::Sender<FileCloseMessage<FileImpl>>,
    ) -> (Self, Receiver<Result<()>>) {
        let (tx_file_close, rx) = tokio::sync::oneshot::channel();
        (
            Self {
                file: Some(file),
                tx_filesystem_close,
                tx_file_close: Some(tx_file_close),
            },
            rx,
        )
    }
}

impl<FileImpl: File> Drop for FileAsyncDrop<FileImpl> {
    fn drop(&mut self) {
        let msg = FileCloseMessage::FileCloseMessage((self.file.take().unwrap(), self.tx_file_close.take().unwrap()));
        if let Err(e) = self.tx_filesystem_close.send(msg) {
            error!("Failed to send FileCloseMessage: {e}");
        }
    }
}

#[async_trait]
impl<FileImpl: File> File for FileAsyncDrop<FileImpl> {
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
}
