use std::sync::Arc;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::sync::oneshot::Receiver;

use crate::actions::copy_file;
use crate::filesystem_async_drop::FilesystemAsyncDrop;
use crate::{File, Filesystem, Result};

struct FileMirror {
    file: Box<dyn File>,
    path: Utf8PathBuf,
    fs1_rx_closed: Receiver<Result<()>>,
}

#[async_trait]
impl File for FileMirror {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        self.file.read_to_end(buf).await
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.file.read(buf).await
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.file.write_all(buf).await
    }
}

/// Applies the same changes to two [`Filesystem`] implementations. Changes are applied on the first
/// one and then mirrored to the second.
///
/// Note.- It´s up to the user to ensure that both [`Filesystem`] instances contain the same files
/// (or the operations running on them, only touch files that are present on both).
pub struct FilesystemMirror<TFilesystem1: Filesystem, TFilesystem2: Filesystem> {
    filesystem1: Option<Arc<TFilesystem1>>,
    filesystem2: Option<Arc<TFilesystem2>>,

    fs_async_drop: FilesystemAsyncDrop<FileMirror>,
}

impl<TFilesystem1: Filesystem + Send + 'static, TFilesystem2: Filesystem + Send + 'static>
    FilesystemMirror<TFilesystem1, TFilesystem2>
{
    pub fn new(filesystem1: TFilesystem1, filesystem2: TFilesystem2) -> Self {
        let filesystem1 = Arc::new(filesystem1);
        let filesystem2 = Arc::new(filesystem2);

        let fs1 = filesystem1.clone();
        let fs2 = filesystem2.clone();
        let fs_async_drop = FilesystemAsyncDrop::new(move |file_mirror: FileMirror| {
            let fs1 = fs1.clone();
            let fs2 = fs2.clone();
            async move {
                // Drop the file so close procedure is triggered in filesystem1
                drop(file_mirror.file);

                // Wait until file is closed in filesystem1
                let _ = file_mirror.fs1_rx_closed.await;

                // Copy contents to filesystem2
                let rx = copy_file(fs1.as_ref(), fs2.as_ref(), &file_mirror.path, &file_mirror.path, true).await?;
                // Wait until file is closed and synced in filesystem2
                let _ = rx.await;

                Ok(())
            }
        });

        Self {
            filesystem1: Some(filesystem1),
            filesystem2: Some(filesystem2),
            fs_async_drop,
        }
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
            .as_ref()
            .unwrap()
            .walk_directory(tx, threads, custom_ignore_filename)
            .await
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Self::Metadata> {
        self.filesystem1.as_ref().unwrap().get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.filesystem1.as_ref().unwrap().exists(path).await
    }

    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Receiver<Result<()>>)> {
        let (file, closed_fs1_rx) = self.filesystem1.as_ref().unwrap().create(path).await?;
        let file_mirror = FileMirror {
            file,
            path: path.to_path_buf(),
            fs1_rx_closed: closed_fs1_rx,
        };
        Ok(self.fs_async_drop.file_wrapped(file_mirror))
    }

    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        // It's read-only, I don't need to do anything on the other filesystem, just return the
        // file object from the "master" filesystem
        self.filesystem1.as_ref().unwrap().open(path).await
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.as_ref().unwrap().create_dir_all(path).await?;
        self.filesystem2.as_ref().unwrap().create_dir_all(path).await
    }

    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.as_ref().unwrap().remove_file(path).await?;
        self.filesystem2.as_ref().unwrap().remove_file(path).await
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.as_ref().unwrap().remove_dir(path).await?;
        self.filesystem2.as_ref().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.filesystem1.as_ref().unwrap().remove_dir_all(path).await?;
        self.filesystem2.as_ref().unwrap().remove_dir_all(path).await
    }

    async fn sync_all(mut self) -> Result<()> {
        // TODO: Execute sync_all in both filesystems.... after closing the thread
        todo!("not imple")
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
        let rx = {
            let (mut f, rx) = fs12.create(&filepath).await.unwrap();
            f.write_all(&content).await.unwrap();
            rx
        };
        let _ = rx.await;

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
