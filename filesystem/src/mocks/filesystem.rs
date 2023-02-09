use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;
use flume::Sender;
use tempfile::{tempdir, TempDir};

use crate::local::FilesystemLocal;
use crate::{File, Filesystem};

/// Mocks a filesystem using a temporal directory that is removed on drop
pub struct FilesystemMock {
    _tmp_dir: TempDir,
    local: FilesystemLocal,
}

impl Default for FilesystemMock {
    fn default() -> Self {
        let tmp_dir = tempdir().unwrap();
        Self {
            local: FilesystemLocal::new(tmp_dir.path()).expect("Temporary directory is not usable!"),
            _tmp_dir: tmp_dir,
        }
    }
}

#[async_trait]
impl Filesystem for FilesystemMock {
    type Metadata = <FilesystemLocal as Filesystem>::Metadata;

    fn root(&self) -> &Path {
        self.local.root()
    }

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Path,
    ) -> Result<()> {
        self.local.walk_directory(tx, threads, custom_ignore_filename).await
    }

    async fn exists(&self, path: &Path) -> Result<bool> {
        self.local.exists(path).await
    }

    async fn create(&self, path: &Path) -> Result<Box<dyn File>> {
        self.local.create(path).await
    }

    async fn open(&self, path: &Path) -> Result<Box<dyn File>> {
        self.local.open(path).await
    }

    async fn create_dir_all(&self, path: &Path) -> Result<()> {
        self.local.create_dir_all(path).await
    }

    async fn remove_file(&self, path: &Path) -> Result<()> {
        self.local.remove_file(path).await
    }

    async fn remove_dir(&self, path: &Path) -> Result<()> {
        self.local.remove_dir(path).await
    }

    async fn remove_dir_all(&self, path: &Path) -> Result<()> {
        self.local.remove_dir_all(path).await
    }
}
