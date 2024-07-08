use std::str::FromStr;

use async_trait::async_trait;
use camino::Utf8PathBuf;
use camino_tempfile::{tempdir, Utf8TempDir};
use flume::Sender;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;

use crate::filesystem::FilesystemOps;
use crate::impls::FilesystemLocal;
use crate::{
    DirectoryPath, Error, File, FileMetadata, FilePath, FilePathBuf, Filename, FilenameBuf, Filesystem, Result,
};

/// Implementation of [`Filesystem`] using a temporal directory in the host filesystem
pub struct FilesystemLocalTemp {
    _tmp_dir: Utf8TempDir,
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

    /// Returns a unique [`FilePathBuf`] inside the given directory. Be aware of typical race conditions
    /// for this operation: another concurrent job taking the same name while this one hasn't used it
    /// already.
    ///
    /// This method will generate the [`FilePathBuf`] using two different strategies:
    /// * If `candidate_filename` is provided, it will try first with the candidate basename and
    ///   extension, and then generate filenames using this pattern: `<basename>_001.<extension>`.
    /// * If no `candidate_filename` is given, it will generate filenames using `<uuid4>.<extension>`
    pub async fn unique_filename(
        &mut self,
        directory_path: &DirectoryPath,
        candidate_filename: Option<&Filename>,
    ) -> Result<FilePathBuf> {
        self.create_dir_all(directory_path).await?;

        let (basename, extension) = match candidate_filename {
            None => (None, "".to_string()),
            Some(candidate) => {
                let ext = candidate.extension().map_or("".to_string(), |v| format!(".{}", v));
                (Some(candidate.basename()), ext)
            }
        };

        let mut attempt = 0;

        let mut create_new_candidate = || {
            let filename = match basename {
                None => {
                    let uuid = uuid::Uuid::new_v4();
                    FilenameBuf::from_str(&format!("{}{}", uuid.to_string().as_str(), extension))?
                }
                Some(basename) => {
                    if attempt == 0 {
                        FilenameBuf::from_str(&format!("{}{}", basename, extension))?
                    } else if attempt > 100 {
                        return Err(Error::Other("Too many retries".to_string()));
                    } else {
                        FilenameBuf::from_str(&format!("{}_{:03}{}", basename, attempt, extension))?
                    }
                }
            };
            attempt += 1;
            Ok(FilePathBuf::new(directory_path, filename))
        };

        let mut filepath = create_new_candidate()?;
        while self.exists(&filepath).await? {
            filepath = create_new_candidate()?;
        }

        Ok(filepath)
    }

    /// Returns the absolute path to the given `filepath`. This path is only valid as long as the
    /// filesystem is not destroyed.
    pub fn resolve_filepath(&self, filepath: impl AsRef<FilePath>) -> Utf8PathBuf {
        self._tmp_dir.path().join(filepath.as_ref())
    }
}

impl Default for FilesystemLocalTemp {
    fn default() -> Self {
        let tmp_dir = tempdir().unwrap();
        Self {
            local: FilesystemLocal::new(tmp_dir.path()).expect("Temporary directory is not usable!"),
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
