use crate::actions::copy_file;
use crate::wrappers::AsyncFileDropImpl;
use crate::{Error, File, FileMetadata, Filesystem, FilesystemRead, FilesystemRemove, FilesystemWrite, Result};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::oneshot::Receiver;
use tokio::sync::Mutex;

/// Function called from [`FilesystemIndexed`] when a file opened in write mode is being dropped.
/// This function calls [`File::sync_all`], drops the `file`, awaits for any pending action in the
/// drop procedure using `rx_filesystem` and finally creates or updates the entry in the index.
async fn index_file<
    LHS: FilesystemRead + FilesystemWrite + 'static,
    RHS: FilesystemRead + FilesystemWrite + 'static,
>(
    file: Box<dyn File>,
    rx_filesystem: Option<Receiver<Result<()>>>,
    index: Arc<Mutex<Option<LHS>>>,
    storage: Arc<Mutex<Option<RHS>>>,
    path: Utf8PathBuf,
) -> Result<()> {
    // Write everything down
    file.sync_all().await?;
    drop(file);
    if let Some(r) = rx_filesystem {
        r.await
            .map_err(|e| Error::Other(format!("Error receiving drop result from rx_filesystem: {e}")))??;
    }

    // Now update or create the entry in the INDEX filesystem (copy from storage to index)
    // FIXME: copy_file is not suitable here. Probably the DB/INDEX implementation only wants the
    // FIXME: metadata (to store in the DB) and it can just get metadata using `fs.get_metadata`
    let index = index.lock().await;
    let storage = storage.lock().await;
    let rx_index = copy_file(
        storage.as_ref().unwrap(),
        index.deref().as_ref().unwrap(),
        &path,
        &path,
        true,
    )
    .await?;

    match rx_index {
        Some(rx_index) => rx_index
            .await
            .map_err(|e| Error::Other(format!("Error receiving drop result from rx_copy_file: {e}")))?,
        None => Ok(()),
    }
}

/// A filesystem implementation that uses an index and a storage. Read operations are done in the
/// index, while write operations run on the storage and are synced to the index afterward.
///
/// This is the typical scenario where there is a database indexing the files in a remote storage.
/// We only want to hit the storage to read the actual content of the files and to save them, but
/// every other operation runs against the database to save time/bandwidth.
pub struct FilesystemIndexed<TIndex: FilesystemRead, TStorage: FilesystemWrite> {
    index: Arc<Mutex<Option<TIndex>>>,
    storage: AsyncFileDropImpl<TStorage>,
}

impl<TIndex: FilesystemWrite + FilesystemRead + 'static, TStorage: FilesystemWrite + FilesystemRead + 'static>
    FilesystemIndexed<TIndex, TStorage>
{
    pub fn new(index: TIndex, storage: TStorage) -> Self {
        let index = Arc::new(Mutex::new(Some(index)));

        let fs_index = index.clone();
        let fs_storage = AsyncFileDropImpl::new(
            storage,
            None,
            Some(
                move |file: Box<dyn File>,
                      rx_filesystem: Option<Receiver<Result<()>>>,
                      fs_storage: Arc<Mutex<Option<TStorage>>>,
                      path: Utf8PathBuf| {
                    // Write everything down
                    let index = index.clone();
                    async move { index_file(file, rx_filesystem, index, fs_storage, path).await }
                },
            ),
        );

        Self {
            index: fs_index,
            storage: fs_storage,
        }
    }

    /// Syncs the contents of both filesystems. In this [`FilesystemIndexed`] it means that all the
    /// files from storage in filesystem2 will be indexed into the filesystem1. Missing files will
    /// be removed from the index.
    pub async fn sync(&self) -> Result<()> {
        todo!("not implemented")
        // FIXME: Implement in terms of some external `action`: backup, sync, mirror,...
    }
}

#[async_trait]
impl<
        TIndex: Filesystem + FilesystemWrite + FilesystemRead,
        TStorage: Filesystem + FilesystemWrite + FilesystemRead,
    > Filesystem for FilesystemIndexed<TIndex, TStorage>
{
    async fn sync_all(self) -> Result<()> {
        self.storage.sync_all().await?;
        self.index.lock().await.take().unwrap().sync_all().await
    }
}

#[async_trait]
impl<TIndex: FilesystemRead, TStorage: FilesystemWrite> FilesystemRead for FilesystemIndexed<TIndex, TStorage> {
    async fn walk_directory(
        &self,
        tx: Sender<Box<dyn FileMetadata>>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.index
            .lock()
            .await
            .as_ref()
            .unwrap()
            .walk_directory(tx, threads, custom_ignore_filename)
            .await
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        self.index.lock().await.as_ref().unwrap().get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.index.lock().await.as_ref().unwrap().exists(path).await
    }

    async fn open(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.index.lock().await.as_ref().unwrap().open(path).await
    }
}

#[async_trait]
impl<TIndex: FilesystemWrite + FilesystemRead, TStorage: FilesystemWrite> FilesystemWrite
    for FilesystemIndexed<TIndex, TStorage>
{
    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        // Only the file in the storage needs to be created, the INDEX will be synced when the
        // file is dropped.
        self.storage.create(path).await
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.storage.create_dir_all(path).await?;
        self.index.lock().await.as_ref().unwrap().create_dir_all(path).await
    }
}

#[async_trait]
impl<TIndex: FilesystemRead + FilesystemWrite + FilesystemRemove, TStorage: FilesystemWrite + FilesystemRemove>
    FilesystemRemove for FilesystemIndexed<TIndex, TStorage>
{
    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_file(path).await?;
        self.index.lock().await.as_ref().unwrap().remove_file(path).await
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_dir(path).await?;
        self.index.lock().await.as_ref().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_dir_all(path).await?;
        self.index.lock().await.as_ref().unwrap().remove_dir_all(path).await
    }
}
