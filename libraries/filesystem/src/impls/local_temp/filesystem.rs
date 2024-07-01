use std::str::FromStr;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use ignore_files::IgnoreFilter;
use tempfile::{tempdir, TempDir};
use tokio::sync::oneshot::Receiver;

use crate::filesystem::FilesystemOps;
use crate::impls::FilesystemLocal;
use crate::{DirectoryPath, File, FileMetadata, FilePath, FilePathBuf, FilenameBuf, Filesystem, Result};

/// Implementation of [`Filesystem`] using a temporal directory in the host filesystem
pub struct FilesystemLocalTemp {
    _tmp_dir: TempDir,
    local: FilesystemLocal,
}

impl FilesystemLocalTemp {
    /// Returns a filename inside the [`FilesystemLocalTemp`] in the root directory.
    ///
    /// User can provide a prefix and suffix for the created filename. This method will add some
    /// randomness (uuid4) to the filename so uniqueness can be assumed.
    pub fn temp_filename(&self, prefix: Option<&str>, suffix: Option<&str>) -> FilePathBuf {
        let uuid = uuid::Uuid::new_v4();
        let filename = format!("{}{}{}", prefix.unwrap_or(""), uuid, suffix.unwrap_or(""));
        FilePathBuf::new(
            DirectoryPath::root(),
            FilenameBuf::from_str(&filename).expect("Generated filename is not valid"),
        )
    }

    /// Returns the absolute path to the given `filepath`. This path is only valid as long as the
    /// filesystem is not destroyed.
    pub fn resolve_filepath(&self, filepath: impl AsRef<FilePath>) -> Utf8PathBuf {
        let root = Utf8Path::from_path(self._tmp_dir.path()).unwrap();
        root.join(filepath.as_ref())
    }
}

impl Default for FilesystemLocalTemp {
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
impl Filesystem for FilesystemLocalTemp {
    async fn sync_all(self) -> Result<()> {
        self.local.sync_all().await
    }

    async fn walk_directory(&self, tx: Sender<Box<dyn FileMetadata>>, ignore_filter: IgnoreFilter) -> Result<()> {
        self.local.walk_directory(tx, ignore_filter).await
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<Box<dyn FileMetadata>> {
        self.local.get_metadata(path).await
    }

    async fn exists(&self, path: &FilePath) -> Result<bool> {
        self.local.exists(path).await
    }
    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.local.open(path).await
    }

    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.local.create(path).await
    }
    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.local.create_dir_all(path).await
    }

    async fn remove_file(&mut self, path: &FilePath) -> Result<()> {
        self.local.remove_file(path).await
    }

    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()> {
        self.local.remove_dir(path).await
    }

    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.local.remove_dir_all(path).await
    }

    async fn internal_copy(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        self.local.internal_copy(origin, target, force).await
    }

    async fn internal_move(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        self.local.internal_move(origin, target, force).await
    }
}

#[async_trait]
impl FilesystemOps for FilesystemLocalTemp {}
