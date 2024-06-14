use std::sync::Arc;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tokio::sync::oneshot::Receiver;
use tokio::sync::Mutex;

use crate::actions::copy_file;
use crate::wrappers::AsyncFileDropImpl;
use crate::{Error, File, FileMetadata, Filesystem, FilesystemRead, FilesystemRemove, FilesystemWrite, Result};

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
    let mut index = index.lock().await;
    let storage = storage.lock().await;
    let rx_index = copy_file(storage.as_ref().unwrap(), index.as_mut().unwrap(), &path, &path, true).await?;

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
    async fn create(&mut self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        // Only the file in the storage needs to be created, the INDEX will be synced when the
        // file is dropped.
        self.storage.create(path).await
    }

    async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        self.storage.create_dir_all(path).await?;
        self.index.lock().await.as_mut().unwrap().create_dir_all(path).await
    }
}

#[async_trait]
impl<TIndex: FilesystemRead + FilesystemWrite + FilesystemRemove, TStorage: FilesystemWrite + FilesystemRemove>
    FilesystemRemove for FilesystemIndexed<TIndex, TStorage>
{
    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_file(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_file(path).await
    }

    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_dir(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_dir(path).await
    }

    async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        self.storage.remove_dir_all(path).await?;
        self.index.lock().await.as_mut().unwrap().remove_dir_all(path).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use camino::{Utf8Component, Utf8PathBuf};

    use crate::Filesystem;

    use super::*;

    /// Something that implements the index
    #[derive(Default)]
    struct IndexImpl {
        /// Maps the path to the files
        files: HashMap<Utf8PathBuf, IndexImplFile>,

        /// Path to directories
        directories: Vec<Utf8PathBuf>,
    }

    #[derive(Debug, Clone)]
    struct IndexImplFile {
        path: Utf8PathBuf,
        hash: String,
        size: u64,
    }

    impl FileMetadata for IndexImplFile {
        fn path(&self) -> &Utf8Path {
            &self.path
        }

        fn size(&self) -> Result<u64> {
            Ok(self.size)
        }

        fn hash(&self) -> Result<String> {
            Ok(self.hash.clone())
        }
    }

    #[async_trait]
    impl Filesystem for IndexImpl {
        async fn sync_all(self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl FilesystemRead for IndexImpl {
        async fn walk_directory(
            &self,
            tx: Sender<Box<dyn FileMetadata>>,
            _threads: usize,
            _custom_ignore_filename: &Utf8Path,
        ) -> Result<()> {
            for it in self.files.values() {
                tx.send(Box::new(it.clone())).unwrap();
            }
            Ok(())
        }

        async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
            match self.files.get(path) {
                None => Err(Error::PathDoesNotExist),
                Some(f) => Ok(Box::new(f.clone())),
            }
        }

        async fn exists(&self, path: &Utf8Path) -> Result<bool> {
            Ok(self.files.contains_key(path) || self.directories.contains(&path.to_path_buf()))
        }

        async fn open(&self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
            Err(Error::Forbidden)
        }
    }

    #[async_trait]
    impl FilesystemWrite for IndexImpl {
        async fn create(&mut self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
            // We can compute the hash and size! We can't introduce it in the database
            Err(Error::Forbidden)
        }

        async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
            let mut current_path = Utf8PathBuf::from("");
            for it in path.components() {
                if let Utf8Component::Normal(p) = it {
                    current_path = current_path.join(p);
                }
                self.directories.push(current_path.clone());
            }
            Ok(())
        }
    }

    #[async_trait]
    impl FilesystemRemove for IndexImpl {
        async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
            let _ = self.files.remove(path);
            Ok(())
        }

        async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
            let found = self
                .directories
                .iter()
                .filter(|d| d.starts_with(path))
                .collect::<Vec<_>>();
            match found.len() {
                0 => Err(Error::PathDoesNotExist),
                1 => {
                    let pos = found.into_iter().position(|p| p.eq(path));
                    match pos {
                        None => Err(Error::PathDoesNotExist),
                        Some(pos) => {
                            self.directories.remove(pos);
                            Ok(())
                        }
                    }
                }
                _ => Err(Error::Other("Multiple found. Directory is not empty".to_string())),
            }
        }

        async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
            let path_with_trailing_slash = Utf8PathBuf::from(path.to_string() + "/");
            let _ = self
                .directories
                .retain(|d| !d.starts_with(&path_with_trailing_slash) && !d.eq(path));
            Ok(())
        }
    }

    #[test]
    fn test_index_impl() {
        let _index = IndexImpl::default();
    }
}
