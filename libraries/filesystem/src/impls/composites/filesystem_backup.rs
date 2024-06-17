use std::sync::Arc;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::sync::oneshot::Receiver;
use tokio::sync::Mutex;

use crate::actions::copy_file;
use crate::wrappers::AsyncFileDropImpl;
use crate::{Error, File, FileMetadata, Filesystem, Result};

/// Function called from [`FilesystemBackup`] when the file from the LHS filesystem is being dropped. This function
/// calls [`File::sync_all`], drops the `file`, awaits for any pending action in the drop procedure using `rx_filesystem`
/// and finally copies the file to the RHS filesystem (using the given `path`).
async fn backup_file<LHS: Filesystem + 'static, RHS: Filesystem + 'static>(
    file: Box<dyn File>,
    rx_filesystem: Option<Receiver<Result<()>>>,
    fs_lhs: Arc<Mutex<Option<LHS>>>,
    fs_rhs: Arc<Mutex<Option<RHS>>>,
    path: Utf8PathBuf,
) -> Result<()> {
    // Write everything down
    file.sync_all().await?;
    drop(file);
    if let Some(r) = rx_filesystem {
        r.await
            .map_err(|e| Error::Other(format!("Error receiving drop result from rx_filesystem: {e}")))??;
    }

    // Now copy from the lhs filesystem to the rhs filesystem
    let fs_lhs = fs_lhs.lock().await;
    let mut fs_rhs = fs_rhs.lock().await;
    let rx_rhs = copy_file(fs_lhs.as_ref().unwrap(), fs_rhs.as_mut().unwrap(), &path, &path, true).await?;

    match rx_rhs {
        Some(rx_rhs) => rx_rhs
            .await
            .map_err(|e| Error::Other(format!("Error receiving drop result from rx_copy_file: {e}")))?,
        None => Ok(()),
    }
}

/// Implements [`Filesystem`] traits operating on two filesystems at the same time. All changes are
/// applied first to the LHS filesystem and then applied to the RHS filesystem:
///  * Read operations only run on the LHS
///  * File write operations run on the LHS and only after the file is closed, it's copied to the
///    RHS filesystem.
///
/// Note.- If the action on the first filesystem fails, the second won't be executed.  If the
/// action fails on the second, the first one won't be rolled back.
///
/// Note.- It´s up to the user to ensure that both [`Filesystem`] instances contain the same files
/// (or the operations running on them only touch files that are present on both). See method
/// [`FilesystemBackup::sync`].
pub struct FilesystemBackup<LHS: Filesystem, RHS: Filesystem> {
    lhs: AsyncFileDropImpl<LHS>,
    rhs: Arc<Mutex<Option<RHS>>>,
}

impl<LHS: Filesystem + 'static, RHS: Filesystem + 'static> FilesystemBackup<LHS, RHS> {
    pub fn new(fs_lhs: LHS, fs_rhs: RHS) -> Self {
        let fs_rhs = Arc::new(Mutex::new(Some(fs_rhs)));

        let filesytem_rhs = fs_rhs.clone();
        let fs_with_create_backup = AsyncFileDropImpl::new(
            fs_lhs,
            None,
            Some(
                move |file: Box<dyn File>,
                      rx_filesystem: Option<Receiver<Result<()>>>,
                      fs_lhs: Arc<Mutex<Option<LHS>>>,
                      path: Utf8PathBuf| {
                    let filesytem_rhs = filesytem_rhs.clone();
                    async move { backup_file(file, rx_filesystem, fs_lhs, filesytem_rhs, path).await }
                },
            ),
        );

        Self {
            lhs: fs_with_create_backup,
            rhs: fs_rhs,
        }
    }

    /// Syncs the contents of both filesystems. In this [`FilesystemBackup`] it means that all the
    /// files from one filesystem will be available in the other and viceversa (running this method
    /// can take a while if many files need to be copied).
    pub async fn sync(&self) -> Result<()> {
        // TODO: Implement a method to do the initial sync. Leverage on some external `action`: backup, sync, mirror,...
        todo!("not impl")
    }
}

#[async_trait]
impl<LHS: Filesystem, RHS: Filesystem> Filesystem for FilesystemBackup<LHS, RHS> {
    async fn sync_all(self) -> Result<()> {
        self.lhs.sync_all().await?;
        self.rhs.lock().await.take().unwrap().sync_all().await
    }

    async fn walk_directory(
        &self,
        tx: Sender<Box<dyn FileMetadata>>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.lhs.walk_directory(tx, threads, custom_ignore_filename).await
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        self.lhs.get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.lhs.exists(path).await
    }

    async fn open(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.lhs.open(path).await
    }

    async fn create(&mut self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        // The file on the RHS filesystem will be created when the returned one is dropped. This is
        // the magic implemented in this FilesystemBackup.
        self.lhs.create(path).await
    }

    async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        self.lhs.create_dir_all(path).await?;
        self.rhs.lock().await.as_mut().unwrap().create_dir_all(path).await
    }

    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        self.lhs.remove_file(path).await?;
        self.rhs.lock().await.as_mut().unwrap().remove_file(path).await
    }

    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        self.lhs.remove_dir(path).await?;
        self.rhs.lock().await.as_mut().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        self.lhs.remove_dir_all(path).await?;
        self.rhs.lock().await.as_mut().unwrap().remove_dir_all(path).await
    }
}

#[cfg(test)]
mod tests {
    use camino::Utf8PathBuf;
    use tempfile::tempdir;

    use crate::impls::FilesystemLocal;

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

        let fs23 = FilesystemBackup::new(fs2, fs3);
        let mut fs12 = FilesystemBackup::new(fs1, fs23);

        // If I work in fs12, changes will be available in fs1, fs2 and fs3

        // - create the directory
        fs12.create_dir_all(&Utf8PathBuf::from("path/to/folder")).await.unwrap();

        // - create a file and write to it
        let filepath = Utf8PathBuf::from("path/to/folder/my_file.txt");
        let content: Vec<u8> = b"Hello, world!".to_vec();
        let rx = {
            let (mut f, rx) = fs12.create(&filepath).await.unwrap();
            f.write_all(&content).await.unwrap();
            rx.unwrap()
        };
        let _ = rx.await.unwrap();

        // TODO: I need to sleep here so the changes are propagated to the other filesystem. Instead, I should return
        // TODO: receiver so I can await on it until all the tasks are done.
        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;

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
