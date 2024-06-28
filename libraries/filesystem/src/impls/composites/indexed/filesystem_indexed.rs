use std::sync::Arc;

use async_trait::async_trait;
use flume::Sender;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;
use tokio::sync::Mutex;

use crate::diff::impls::mirror;
use crate::filesystem::FilesystemOps;
use crate::wrappers::AsyncFileDropImpl;
use crate::{DirectoryPath, Error, File, FileMetadata, FilePath, FilePathBuf, Filesystem, Result};

/// Function called from [`FilesystemIndexed`] when a file opened in write mode is being dropped.
/// This function calls [`File::sync_all`], drops the `file`, awaits for any pending action in the
/// drop procedure using `rx_filesystem` and finally creates or updates the entry in the index.
async fn index_file<LHS: Filesystem + FilesystemOps + 'static, RHS: Filesystem + 'static>(
    file: Box<dyn File>,
    rx_filesystem: Option<Receiver<Result<()>>>,
    index: Arc<Mutex<Option<LHS>>>,
    storage: Arc<Mutex<Option<RHS>>>,
    path: &FilePath,
) -> Result<()> {
    // Write everything down
    file.sync_all().await?;
    drop(file);
    if let Some(r) = rx_filesystem {
        r.await
            .map_err(|e| Error::Other(format!("Error receiving drop result from rx_filesystem: {e}")))??;
    }

    // Now update or create the entry in the INDEX filesystem (copy from storage to index)
    let mut index = index.lock().await;
    let storage = storage.lock().await;
    let rx_index = index
        .as_mut()
        .unwrap()
        .copy_from(path, storage.as_ref().unwrap(), path, true)
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
pub struct FilesystemIndexed<TIndex: Filesystem, TStorage: Filesystem> {
    index: Arc<Mutex<Option<TIndex>>>,
    storage: AsyncFileDropImpl<TStorage>,
}

impl<TIndex: Filesystem + FilesystemOps + 'static, TStorage: Filesystem + 'static> FilesystemIndexed<TIndex, TStorage> {
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
                      path: FilePathBuf| {
                    // Write everything down
                    let index = index.clone();
                    async move { index_file(file, rx_filesystem, index, fs_storage, &path).await }
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
    pub async fn initial_sync(&self) -> Result<()> {
        mirror(&self.storage, self.index.lock().await.as_mut().unwrap()).await
    }
}

#[async_trait]
impl<TIndex: Filesystem, TStorage: Filesystem> Filesystem for FilesystemIndexed<TIndex, TStorage> {
    async fn sync_all(self) -> Result<()> {
        self.storage.sync_all().await?;
        self.index.lock().await.take().unwrap().sync_all().await
    }

    async fn walk_directory(&self, tx: Sender<Box<dyn FileMetadata>>, ignore_filter: IgnoreFilter) -> Result<()> {
        self.index
            .lock()
            .await
            .as_ref()
            .unwrap()
            .walk_directory(tx, ignore_filter)
            .await
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<Box<dyn FileMetadata>> {
        self.index.lock().await.as_ref().unwrap().get_metadata(path).await
    }

    async fn exists(&self, path: &FilePath) -> Result<bool> {
        self.index.lock().await.as_ref().unwrap().exists(path).await
    }

    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.storage.open(path).await
    }

    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        // Only the file in the storage needs to be created, the INDEX will be synced when the
        // file is dropped.
        self.storage.create(path).await
    }

    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.storage.create_dir_all(path).await?;
        self.index.lock().await.as_mut().unwrap().create_dir_all(path).await
    }

    async fn remove_file(&mut self, path: &FilePath) -> Result<()> {
        self.storage.remove_file(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_file(path).await
    }

    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()> {
        self.storage.remove_dir(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.storage.remove_dir_all(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_dir_all(path).await
    }
}

#[async_trait]
impl<TIndex: Filesystem, TStorage: Filesystem> FilesystemOps for FilesystemIndexed<TIndex, TStorage> {}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::str::FromStr;
    use std::sync::{Arc, RwLock};

    use ignore_files::IgnoreFilter;

    use crate::impls::composites::FilesystemIndexed;
    use crate::impls::mocks::{FilesystemMock, SUCCESS};
    use crate::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf, Filesystem};

    #[tokio::test]
    async fn test_indexed_impl() {
        // Here I just want to test that READ operations run on the index while write ones run on
        // the storage and are synced to the index afterward. I'm using a [`FilesystemMock`] as
        // an index, that will raise an error whenever is hit. As the storage I'm using a local
        // filesystem in a temporal directory, so I can use regular `std::fs` to check if some
        // operations did happened in the host.
        let index_called = Arc::new(RwLock::new(Vec::new()));
        let storage_called = Arc::new(RwLock::new(Vec::new()));
        let mut indexed_filesystem = {
            let index = FilesystemMock::new("index", index_called.clone());
            let storage = FilesystemMock::new("storage", storage_called.clone());
            FilesystemIndexed::new(index, storage)
        };

        let filepath = FilePathBuf::new(&DirectoryPathBuf::root(), FilenameBuf::from_str("path").unwrap());

        // walk_directory
        {
            let (tx, _) = flume::bounded::<Box<dyn FileMetadata>>(0);
            let r = indexed_filesystem.walk_directory(tx, IgnoreFilter::empty("")).await;
            assert!(r.is_err());
            // index was called
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("walk_directory".to_string(), vec![])]
            );
            // storage was not hit
            assert!(storage_called.read().unwrap().is_empty());
        };
        index_called.write().unwrap().clear();

        // get_metadata
        {
            let r = indexed_filesystem.get_metadata(&filepath).await;
            assert!(r.is_err());
            // index was called
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("get_metadata".to_string(), vec!["path".to_string()])]
            );
            // storage was not hit
            assert!(storage_called.read().unwrap().is_empty());
        };
        index_called.write().unwrap().clear();

        // exists
        {
            let r = indexed_filesystem.exists(&filepath).await;
            assert!(r.is_err());
            // index was called
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("exists".to_string(), vec!["path".to_string()])]
            );
            // storage was not hit
            assert!(storage_called.read().unwrap().is_empty());
        };
        index_called.write().unwrap().clear();

        // open
        {
            let r = indexed_filesystem.open(&filepath).await;
            assert!(r.is_err());
            // index was not hit (we don't check if the file exists first)
            assert!(index_called.read().unwrap().is_empty());
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("open".to_string(), vec!["path".to_string()])]
            );
        };
        storage_called.write().unwrap().clear();

        // create
        {
            let r = indexed_filesystem.create(&filepath).await;
            assert!(r.is_err());
            // index was not hit
            assert!(index_called.read().unwrap().is_empty());
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("create".to_string(), vec!["path".to_string()])]
            );
        };
        storage_called.write().unwrap().clear();

        let success_directory = DirectoryPathBuf::from_str(SUCCESS).unwrap();
        // create_dir_all
        {
            let r = indexed_filesystem.create_dir_all(&success_directory).await;
            assert!(r.is_ok());
            // index was hit
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("create_dir_all".to_string(), vec![SUCCESS.to_string()])]
            );
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("create_dir_all".to_string(), vec![SUCCESS.to_string()])]
            );
        };
        index_called.write().unwrap().clear();
        storage_called.write().unwrap().clear();

        let success_file = FilePathBuf::new(DirectoryPathBuf::root(), FilenameBuf::from_str(SUCCESS).unwrap());
        // remove_file
        {
            let r = indexed_filesystem.remove_file(&success_file).await;
            assert!(r.is_ok());
            // index was hit
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("remove_file".to_string(), vec![SUCCESS.to_string()])]
            );
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("remove_file".to_string(), vec![SUCCESS.to_string()])]
            );
        };
        index_called.write().unwrap().clear();
        storage_called.write().unwrap().clear();

        // remove_dir
        {
            let r = indexed_filesystem.remove_dir(&success_directory).await;
            assert!(r.is_ok());
            // index was hit
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("remove_dir".to_string(), vec![SUCCESS.to_string()])]
            );
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("remove_dir".to_string(), vec![SUCCESS.to_string()])]
            );
        };
        index_called.write().unwrap().clear();
        storage_called.write().unwrap().clear();

        // remove_dir_all
        {
            let r = indexed_filesystem.remove_dir_all(&success_directory).await;
            assert!(r.is_ok());
            // index was hit
            assert_eq!(
                index_called.read().unwrap().deref(),
                &vec![("remove_dir_all".to_string(), vec![SUCCESS.to_string()])]
            );
            // storage is returning the file
            assert_eq!(
                storage_called.read().unwrap().deref(),
                &vec![("remove_dir_all".to_string(), vec![SUCCESS.to_string()])]
            );
        };
        index_called.write().unwrap().clear();
        storage_called.write().unwrap().clear();
    }
}
