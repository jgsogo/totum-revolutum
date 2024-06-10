use std::sync::Arc;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::task::JoinHandle;
use tracing::{error, info};

use crate::actions::copy_file;
use crate::{Error, File, Filesystem, Result};

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
    filesystem1: Arc<TFilesystem1>,
    filesystem2: Arc<TFilesystem2>,

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

impl<TFilesystem1: Filesystem + Send + 'static, TFilesystem2: Filesystem + Send + 'static>
    FilesystemMirror<TFilesystem1, TFilesystem2>
{
    pub fn new(filesystem1: TFilesystem1, filesystem2: TFilesystem2) -> Self {
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();

        let filesystem1 = Arc::new(filesystem1);
        let filesystem2 = Arc::new(filesystem2);
        let fs1 = filesystem1.clone();
        let fs2 = filesystem2.clone();
        let t = tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileCloseMessage(path) => {
                        // Copy/override file to the second filesystem
                        if let Err(e) = copy_file(fs1.as_ref(), fs2.as_ref(), &path, &path, true).await {
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
impl<TFilesystem1: Filesystem + Send, TFilesystem2: Filesystem + Send> Filesystem
    for FilesystemMirror<TFilesystem1, TFilesystem2>
{
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
        // It's read-only, I don't need to do anything on the other filesystem, just return the
        // file object from the "master" filesystem
        self.filesystem1.open(path).await
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

#[cfg(test)]
mod tests {
    use std::time;
    use tempfile::tempdir;

    use crate::local::FilesystemLocal;

    use super::*;

    #[tokio::test]
    async fn test_create_write_read() {
        let tmp = tempdir().unwrap();
        let root = Utf8Path::from_path(tmp.path()).unwrap();
        let (fs1, fs1_root) = {
            let path = root.join("fs1");
            std::fs::create_dir_all(&path).unwrap();
            (FilesystemLocal::new(&path).unwrap(), path)
        };
        let (fs2, fs2_root) = {
            let path = root.join("fs2");
            std::fs::create_dir_all(&path).unwrap();
            (FilesystemLocal::new(&path).unwrap(), path)
        };
        let (fs3, fs3_root) = {
            let path = root.join("fs3");
            std::fs::create_dir_all(&path).unwrap();
            (FilesystemLocal::new(&path).unwrap(), path)
        };

        let fs23 = FilesystemMirror::new(fs2, fs3);
        let fs12 = FilesystemMirror::new(fs1, fs23);

        // If I work in fs12, changes will be available in fs1, fs2 and fs3

        // - create the directory
        fs12.create_dir_all(&Utf8PathBuf::from("path/to/folder")).await.unwrap();

        // - create a file and write to it
        let filepath = Utf8PathBuf::from("path/to/folder/my_file.txt");
        let content: Vec<u8> = b"Hello, world!".to_vec();
        {
            let mut f = fs12.create(&filepath).await.unwrap();
            f.write_all(&content).await.unwrap();
        }

        // TODO: I need to sleep here so the changes are propagated to the other filesystem. Instead, I should return
        // TODO: receiver so I can await on it until all the tasks are done.
        tokio::time::sleep(time::Duration::from_millis(2000)).await;

        let fs1_filepath = fs1_root.join(&filepath);
        let c1 = std::fs::read(&fs1_filepath).unwrap();
        assert_eq!(content, c1);
        let fs2_filepath = fs2_root.join(&filepath);
        let c2 = std::fs::read(&fs2_filepath).unwrap();
        assert_eq!(content, c2);
        let fs3_filepath = fs3_root.join(&filepath);
        let c3 = std::fs::read(&fs3_filepath).unwrap();
        assert_eq!(content, c3);

        // - remove the file
        fs12.remove_file(&filepath).await.unwrap();
        assert!(!fs12.exists(&filepath).await.unwrap());
        assert!(!fs1_filepath.exists());
        assert!(!fs2_filepath.exists());
        assert!(!fs3_filepath.exists());

        // - remove the directories
        fs12.remove_dir_all(&Utf8PathBuf::from("path")).await.unwrap();
        assert!(!fs1_root.join("path").exists());
        assert!(!fs2_root.join("path").exists());
        assert!(!fs3_root.join("path").exists());
    }
}
