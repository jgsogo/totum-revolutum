use std::str::FromStr;

use async_trait::async_trait;
use camino::Utf8PathBuf;
use camino_tempfile::{tempdir, Utf8TempDir};
use flume::Sender;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;

use crate::filesystem::FilesystemOps;
use crate::impls::FilesystemLocalSync;
use crate::{DirectoryPath, File, FileMetadata, FilePath, FilePathBuf, FilenameBuf, Filesystem, Result};

/// Implementation of [`Filesystem`] using a temporal directory in the host filesystem
pub struct FilesystemLocalTemp {
    _tmp_dir: Utf8TempDir,
    local: FilesystemLocalSync,
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

    /// Returns the absolute path to the given [`FilePath`]. This path is only valid as long as the
    /// filesystem is not destroyed.
    pub fn resolve_filepath(&self, filepath: impl AsRef<FilePath>) -> Utf8PathBuf {
        self.local.resolve_filepath(filepath)
    }
}

impl Default for FilesystemLocalTemp {
    fn default() -> Self {
        let tmp_dir = tempdir().unwrap();
        Self {
            local: FilesystemLocalSync::new(tmp_dir.path()).expect("Temporary directory is not usable!"),
            _tmp_dir: tmp_dir,
        }
    }
}

#[async_trait]
impl Filesystem for FilesystemLocalTemp {
    async fn sync_all(self) -> Result<()> {
        self.local.sync_all().await
    }

    async fn walk_directory(&self, tx: Sender<FileMetadata>, ignore_filter: IgnoreFilter) -> Result<()> {
        self.local.walk_directory(tx, ignore_filter).await
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<FileMetadata> {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::impls::FilesystemLocalTemp;
    use crate::{DirectoryPathBuf, FilenameBuf, Filesystem};

    #[tokio::test]
    async fn test_unique_filename() {
        let mut fs = FilesystemLocalTemp::default();

        let directory_path = DirectoryPathBuf::from_str("a/b/c").unwrap();

        // No candidate filename
        let f1 = fs.unique_filename(&directory_path, None).await.unwrap();
        assert_eq!(f1.directory().as_str(), directory_path.as_str());
        assert!(uuid::Uuid::parse_str(f1.filename().as_str()).ok().is_some());

        // Candidate without extension
        let filename_buf = FilenameBuf::from_str("candidate").unwrap();
        let f1 = fs.unique_filename(&directory_path, Some(&filename_buf)).await.unwrap();
        assert_eq!(f1.filename().as_str(), "candidate");
        fs.create(&f1).await.unwrap();
        let f2 = fs.unique_filename(&directory_path, Some(&filename_buf)).await.unwrap();
        assert_eq!(f2.filename().as_str(), "candidate_001");

        // Candidate with extension
        let filename_buf = FilenameBuf::from_str("candidate.ext").unwrap();
        let f1 = fs.unique_filename(&directory_path, Some(&filename_buf)).await.unwrap();
        assert_eq!(f1.filename().as_str(), "candidate.ext");
        fs.create(&f1).await.unwrap();
        let f2 = fs.unique_filename(&directory_path, Some(&filename_buf)).await.unwrap();
        assert_eq!(f2.filename().as_str(), "candidate_001.ext");
    }

    #[test]
    fn test_temp_filename() {
        let fs = FilesystemLocalTemp::default();

        let f1 = fs.temp_filename(None, None);
        assert_eq!(f1.directory().as_str(), "");
        assert!(uuid::Uuid::parse_str(f1.filename().as_str()).ok().is_some());

        let f1 = fs.temp_filename(Some("prefix"), None);
        assert_eq!(f1.directory().as_str(), "");
        let rest = f1.filename().as_str().strip_prefix("prefix").unwrap();
        assert!(uuid::Uuid::parse_str(rest).ok().is_some());

        let f1 = fs.temp_filename(None, Some("suffix"));
        assert_eq!(f1.directory().as_str(), "");
        let rest = f1.filename().as_str().strip_suffix("suffix").unwrap();
        assert!(uuid::Uuid::parse_str(rest).ok().is_some());

        let f1 = fs.temp_filename(Some("prefix"), Some("suffix"));
        assert_eq!(f1.directory().as_str(), "");
        let rest = f1.filename().as_str().strip_prefix("prefix").unwrap();
        let rest = rest.strip_suffix("suffix").unwrap();
        assert!(uuid::Uuid::parse_str(rest).ok().is_some());
    }
}
