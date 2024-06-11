use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use tempfile::{tempdir, TempDir};
use tokio::sync::oneshot::Receiver;

use crate::impls::FilesystemLocal;
use crate::{File, FileMetadata, Filesystem};
use crate::{FilesystemRead, FilesystemRemove, FilesystemWrite, Result};

/// Implementation of [`Filesystem`] using a temporal directory in the host filesystem
pub struct FilesystemLocalTemp {
    tmp_dir: TempDir,
    local: FilesystemLocal,
}

impl FilesystemLocalTemp {
    /// Returns a filename inside the [`FilesystemLocalTemp`]
    ///
    /// User can provide a prefix and suffix for the created filename. This method will add some
    /// randomness (uuid4) to the filename so uniqueness can be assumed.
    pub fn temp_filename(&self, prefix: Option<&str>, suffix: Option<&str>) -> Utf8PathBuf {
        let uuid = uuid::Uuid::new_v4();
        let filename = format!("{}{}{}", prefix.unwrap_or(""), uuid, suffix.unwrap_or(""));
        Utf8PathBuf::from_path_buf(self.tmp_dir.as_ref().join(filename)).expect("Failed to generate UTF8 path")
    }
}

impl Default for FilesystemLocalTemp {
    fn default() -> Self {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        Self {
            local: FilesystemLocal::new(utf8_path).expect("Temporary directory is not usable!"),
            tmp_dir,
        }
    }
}

#[async_trait]
impl Filesystem for FilesystemLocalTemp {
    async fn sync_all(self) -> Result<()> {
        self.local.sync_all().await
    }
}

#[async_trait]
impl FilesystemWrite for FilesystemLocalTemp {
    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.local.create(path).await
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        self.local.create_dir_all(path).await
    }
}

#[async_trait]
impl FilesystemRead for FilesystemLocalTemp {
    async fn walk_directory(
        &self,
        tx: Sender<Box<dyn FileMetadata>>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        self.local.walk_directory(tx, threads, custom_ignore_filename).await
    }
    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        self.local.get_metadata(path).await
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        self.local.exists(path).await
    }
    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        self.local.open(path).await
    }
}

#[async_trait]
impl FilesystemRemove for FilesystemLocalTemp {
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
