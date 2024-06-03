use async_trait::async_trait;
use camino::Utf8Path;
use flume::Sender;
use tempfile::{tempdir, TempDir};

use crate::local::FilesystemLocal;
use crate::Result;
use crate::{File, Filesystem};

/// Mocks a filesystem using a temporal directory that is removed on drop
pub struct FilesystemMock {
    _tmp_dir: TempDir,
    local: FilesystemLocal,
}

impl Default for FilesystemMock {
    fn default() -> Self {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        Self {
            local: FilesystemLocal::new(utf8_path).expect("Temporary directory is not usable!"),
            _tmp_dir: tmp_dir,
        }
    }
}

#[async_trait]
impl Filesystem for FilesystemMock {
    type Metadata = <FilesystemLocal as Filesystem>::Metadata;

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.local.walk_directory(tx, threads, custom_ignore_filename).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.local.exists(path).await
    }

    async fn create(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        self.local.create(path).await
    }

    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        self.local.open(path).await
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.local.create_dir_all(path).await
    }

    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        self.local.remove_file(path).await
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        self.local.remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.local.remove_dir_all(path).await
    }
}
