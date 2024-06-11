use crate::{File, Filesystem, FilesystemAsyncDrop};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(dead_code)]
struct FileToIndex;

#[async_trait]
impl File for FileToIndex {
    async fn read_to_end(&mut self, _buf: &mut Vec<u8>) -> crate::Result<usize> {
        todo!()
    }

    async fn read(&mut self, _buf: &mut [u8]) -> crate::Result<usize> {
        todo!()
    }

    async fn write_all(&mut self, _buf: &[u8]) -> crate::Result<()> {
        todo!()
    }
}

/// A filesystem implementation that uses an index and a storage. Read operations are done in the
/// index, while write operations run on the storage and are synced to the index afterward.
///
/// This is the typical scenario where there is a database indexing the files in a remote storage.
/// We only want to hit the storage to read the actual content of the files and to save them, but
/// every other operation runs against the database to save time/bandwidth.
///
/// This implementation is very similar to [`crate::impls::composites::FilesystemMirror`], it only changes the
/// preferred filesystem used depending on the operation.
pub struct FilesystemIndexed<TFilesystem1: Filesystem, TFilesystem2: Filesystem> {
    _index: Arc<Mutex<Option<TFilesystem1>>>,
    _storage: Arc<Mutex<Option<TFilesystem2>>>,

    _fs_async_drop: FilesystemAsyncDrop<FileToIndex>,
}

impl<TFilesystem1: Filesystem, TFilesystem2: Filesystem> FilesystemIndexed<TFilesystem1, TFilesystem2> {
    pub fn new(_index: TFilesystem1, _storage: TFilesystem2) -> Self {
        todo!("not impl")
    }

    /// Syncs the contents of both filesystems. In this [`FilesystemIndexed`] it means that all the
    /// files from storage in filesystem2 will be indexed into the filesystem1. Missing files will
    /// be removed from the index.
    pub fn sync(&self) -> crate::Result<()> {
        todo!("not implemented")
        // FIXME: Implement in terms of some external `action`: backup, sync, mirror,...
    }
}
