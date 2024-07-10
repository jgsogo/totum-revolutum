use std::fs;
use std::str::FromStr;

use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use ignore::WalkBuilder;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;
use tokio::time::Instant;
use tracing::{error, info};

use crate::filesystem::FilesystemOps;
use crate::ignore_filter::IgnoreFilterT;
use crate::paths::FilesystemPath;
use crate::{
    DirectoryPath, DirectoryPathBuf, Error, File, FileMetadata, FilePath, FilePathBuf, FilenameBuf, Filesystem, Result,
};

use super::parallel_visitor;

/// Implementation of [`Filesystem`] using a directory in the host filesystem.
#[derive(Debug, Clone)]
pub struct FilesystemLocal {
    root: Utf8PathBuf,
}

impl FilesystemLocal {
    /// Creates a new [`FilesystemLocal`] at the given `root` path.
    pub fn new(root: &Utf8Path) -> Result<Self> {
        if !root.exists() {
            return Err(Error::PathDoesNotExist);
        }

        Ok(Self {
            root: root.to_path_buf(),
        })
    }

    /// Creates a new [`FilesystemLocal`] at the `root` of the current hard disk (`/`)
    pub fn local_hd() -> Self {
        Self {
            root: Utf8PathBuf::from_str("/").unwrap(),
        }
    }

    /// Takes an absolute path (it will convert it to absolute one using [`utils::filesystem::to_absolute_path`]
    /// if needed) and returns the corresponding path inside `self` filesystem. If the path doesn't
    /// exist it will return a [`Error::IoError`] error.
    pub fn relativize_path<P: AsRef<Utf8Path>>(&self, path: P) -> Result<FilesystemPath> {
        let path = utils::filesystem::to_absolute_path(path);
        let rel_path = path
            .strip_prefix(&self.root)
            .map_err(|_| Error::Other("Input path is not inside this filesystem".to_string()))?;

        let fs_metadata = std::fs::metadata(path.as_std_path())?;
        if fs_metadata.is_dir() {
            Ok(FilesystemPath::DirectoryPath(DirectoryPathBuf::try_from(rel_path)?))
        } else {
            let filename = rel_path.file_name().expect("It should contain at least the filename");
            let filename = FilenameBuf::from_str(filename)?;
            let filepath = match rel_path.parent() {
                None => FilePathBuf::new(DirectoryPathBuf::root(), filename),
                Some(d) => {
                    let directory = DirectoryPathBuf::try_from(d)?;
                    FilePathBuf::new(directory, filename)
                }
            };
            Ok(FilesystemPath::FilePath(filepath))
        }
    }

    /// Returns the absolute path to the given [`FilePath`].
    pub fn resolve_filepath(&self, filepath: impl AsRef<FilePath>) -> Utf8PathBuf {
        self.root.join(filepath.as_ref())
    }
}

#[async_trait]
impl Filesystem for FilesystemLocal {
    async fn sync_all(mut self) -> Result<()> {
        Ok(())
    }

    async fn walk_directory(&self, tx: Sender<FileMetadata>, ignore_filter: IgnoreFilter) -> Result<()> {
        let root = self.root.clone();

        let walker = WalkBuilder::new(&self.root)
            .threads(4) // TODO: How to configure this default? Builder patter that accepts this init value?
            .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
            .filter_entry(move |entry| {
                if let Some(file_type) = entry.file_type() {
                    let relative_path = entry
                        .path()
                        .strip_prefix(&root)
                        .expect("File not contained inside root!");
                    let directory_path =
                        unsafe { DirectoryPath::assume_valid(Utf8Path::from_path(relative_path).unwrap()) };

                    if file_type.is_dir() {
                        ignore_filter.visit_directory(directory_path)
                    } else {
                        let (parent_dir, last_cmp) = directory_path.split_parent();
                        let parent_dir = parent_dir.unwrap_or(DirectoryPath::root());
                        match FilenameBuf::fix_and_create(last_cmp.unwrap().as_str()) {
                            Ok(filename) => {
                                let filepath = FilePathBuf::new(parent_dir, filename);
                                ignore_filter.visit_file(&filepath)
                            }
                            Err(Error::InvalidFilename) => {
                                error!("Invalid filename: {}", last_cmp.unwrap().as_str());
                                false
                            }
                            Err(_) => false,
                        }
                    }
                } else {
                    false
                }
            })
            .build_parallel();

        info!("Start local visitor");
        let start = Instant::now();
        let mut builder = parallel_visitor::VisitorBuilder::new(tx, self.root.clone());
        walker.visit(&mut builder);
        info!("Finished local visitor in {:?}", start.elapsed());
        Ok(())
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<FileMetadata> {
        let abs_path = self.root.join(path);
        let size = abs_path.metadata().map_err(Error::IoError)?.len();
        let hash = sha256::try_digest(abs_path)
            .map_err(|e| Error::Other(format!("Cannot compute sha256 of given file: {}", e)))?;

        Ok(FileMetadata {
            path: path.to_filepath_buf(),
            hash,
            size,
        })
    }

    async fn exists(&self, path: &FilePath) -> Result<bool> {
        let path = self.root.join(path);
        Ok(path.exists())
    }
    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        let path = self.root.join(path);
        let f = AsyncFile::open(path.into_std_path_buf()).await?;
        Ok((Box::new(f), None))
    }

    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        let path = self.root.join(path);
        let f = AsyncFile::create(path.into_std_path_buf()).await?;
        Ok((Box::new(f), None))
    }
    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        let path = self.root.join(path);
        fs::create_dir_all(path).map_err(Error::IoError)
    }

    async fn remove_file(&mut self, path: &FilePath) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(path);
        fs::remove_file(path).map_err(Error::IoError)
    }

    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(path);
        fs::remove_dir(path).map_err(Error::IoError)
    }

    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(path);
        fs::remove_dir_all(path).map_err(Error::IoError)
    }

    async fn internal_copy(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if !force && self.exists(target).await? {
            return Err(Error::TargetFileExists);
        }
        let origin = self.root.join(origin);
        let target = self.root.join(target);
        let _ = fs::copy(origin, target).map_err(Error::IoError)?;
        Ok(None)
    }

    async fn internal_move(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if !force && self.exists(target).await? {
            return Err(Error::TargetFileExists);
        }
        let origin = self.root.join(origin);
        let target = self.root.join(target);
        fs::rename(origin, target).map_err(Error::IoError)?;
        Ok(None)
    }
}

#[async_trait]
impl FilesystemOps for FilesystemLocal {}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Write;
    use std::str::FromStr;

    use camino_tempfile::{tempdir, NamedUtf8TempFile};

    use crate::{DirectoryPathBuf, FilePathBuf, FilenameBuf};

    use super::*;

    async fn create_file(fs: &mut FilesystemLocal, path: &FilePath, content: &[u8]) -> Result<()> {
        let rx = {
            let (mut f, rx) = fs.create(&path).await?;
            f.write_all(&content).await?;
            rx
        };
        if let Some(rx) = rx {
            rx.await.unwrap().unwrap();
        }
        Ok(())
    }

    #[test]
    fn test_root_not_exists() {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = tmp_dir.path();
        let r = FilesystemLocal::new(&utf8_path.join("not-exist"));
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), Error::PathDoesNotExist))
    }

    #[tokio::test]
    async fn test_root() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = tmp_dir.path();
        let fs = FilesystemLocal::new(utf8_path)?;
        // Root is not canonical, it fails in macOS where tmp directories are inside sym folder
        #[cfg(target_os = "macos")]
        assert_ne!(fs::canonicalize(tmp_dir.path())?, fs.root);
        assert_eq!(tmp_dir.path(), fs.root);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_write_read() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = tmp_dir.path();
        let mut fs = FilesystemLocal::new(utf8_path)?;

        let directory_path = DirectoryPathBuf::root();
        let filepath = FilePathBuf::new(directory_path, FilenameBuf::from_str("myfile").unwrap());
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Create and write
        let rx = {
            let (mut f, rx) = fs.create(&filepath).await?;
            f.write_all(&content).await?;
            rx
        };
        if let Some(rx) = rx {
            rx.await.unwrap().unwrap();
        }

        // Open and read
        {
            let (mut file, _) = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(content, content_read);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_in_subfolder() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = tmp_dir.path();
        let mut fs = FilesystemLocal::new(utf8_path)?;

        let dir = DirectoryPathBuf::from_str("nested/nested2").unwrap();
        let filepath = FilePathBuf::new(&dir, FilenameBuf::from_str("myfile.txt").unwrap());
        // I can't create a file in a subfolder (if the folder doesn't exist yet)
        let r = fs.create(&filepath).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::IoError(io::Error { .. })));

        fs.create_dir_all(&dir).await?;
        let r = fs.create(&filepath).await;
        assert!(r.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_walk_directory() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = tmp_dir.path();
        let mut fs = FilesystemLocal::new(utf8_path)?;

        // TODO: Some kind of fixture that could be reused by "all" the tests would be great
        fs.create_dir_all(&DirectoryPathBuf::from_str("a/path/to/some/folder")?)
            .await?;
        fs.create_dir_all(&DirectoryPathBuf::from_str("a/path/to/another/folder")?)
            .await?;
        fs.create_dir_all(&DirectoryPathBuf::from_str("another/path/to/some/folder")?)
            .await?;

        let f1 = FilenameBuf::from_str("file.txt")?;
        let f2 = FilenameBuf::from_str("other.txt")?;
        let f3 = FilenameBuf::from_str("file.rs")?;
        let f4 = FilenameBuf::from_str("other.rs")?;
        create_file(&mut fs, &FilePathBuf::new(DirectoryPathBuf::from_str("")?, &f1), b"").await?;
        create_file(&mut fs, &FilePathBuf::new(DirectoryPathBuf::from_str("")?, &f2), b"").await?;
        create_file(&mut fs, &FilePathBuf::new(DirectoryPathBuf::from_str("")?, &f3), b"").await?;
        create_file(&mut fs, &FilePathBuf::new(DirectoryPathBuf::from_str("")?, &f4), b"").await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("a/path/to/some/folder")?, &f1),
            b"",
        )
        .await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("a/path/to/some/folder")?, &f3),
            b"",
        )
        .await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("a/path/to/another/folder")?, &f2),
            b"",
        )
        .await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("a/path/to/another/folder")?, &f4),
            b"",
        )
        .await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("another/path/to")?, &f1),
            b"",
        )
        .await?;
        create_file(
            &mut fs,
            &FilePathBuf::new(DirectoryPathBuf::from_str("another/path/to")?, &f3),
            b"",
        )
        .await?;

        // Ignore all "*.rs" files
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["*.rs"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();
            assert_eq!(
                all_files_str,
                vec![
                    "a/path/to/another/folder/other.txt",
                    "a/path/to/some/folder/file.txt",
                    "another/path/to/file.txt",
                    "file.txt",
                    "other.txt"
                ]
            );
        }

        // Ignore everything inside "another/" directory (also if nested)
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["another/"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();
            assert_eq!(
                all_files_str,
                vec![
                    "a/path/to/some/folder/file.rs",
                    "a/path/to/some/folder/file.txt",
                    "file.rs",
                    "file.txt",
                    "other.rs",
                    "other.txt"
                ]
            );
        }

        // Ignore everything inside "another/" directory (only if root), and ignore all `.rs` files
        // inside a/path
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["/another/", "a/path/**/*.rs"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();
            assert_eq!(
                all_files_str,
                vec![
                    "a/path/to/another/folder/other.txt",
                    "a/path/to/some/folder/file.txt",
                    "file.rs",
                    "file.txt",
                    "other.rs",
                    "other.txt"
                ]
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_local_hd() -> Result<()> {
        // We create a temporary file in the HD
        let mut tmpfile = NamedUtf8TempFile::new()?;
        tmpfile.write_all(b"Something")?;

        // Get the FilePath to that file (relative path)
        let filepath = {
            let relative_path_to_root = tmpfile.path().to_path_buf().canonicalize().unwrap();
            let directory = DirectoryPathBuf::from_str(
                relative_path_to_root
                    .parent()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .strip_prefix("/")
                    .unwrap(),
            )?;
            let filename = FilenameBuf::from_str(relative_path_to_root.file_name().unwrap().to_str().unwrap())?;
            FilePathBuf::new(directory, filename)
        };

        // We can access the file using a filesystem instantiated with FilesystemLocal::local_hd()
        let local_hd = FilesystemLocal::local_hd();
        let (mut file, _) = local_hd.open(&filepath).await?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        assert_eq!(buffer.as_slice(), b"Something");
        Ok(())
    }

    #[tokio::test]
    async fn test_relative_path() -> Result<()> {
        let tmp_path = tempdir()?;
        let mut fs = FilesystemLocal::new(tmp_path.path())?;

        {
            let r = fs.relativize_path(tmp_path.path().join("a/path/to/something"));
            assert!(r.is_err());
            assert!(matches!(r.unwrap_err(), Error::IoError(..)));
        }

        {
            let dirpath = DirectoryPathBuf::from_str("a/path/to")?;
            fs.create_dir_all(&dirpath).await?;
            let filename = FilenameBuf::from_str("something")?;
            let filepath = FilePathBuf::new(dirpath, filename);
            fs.create(&filepath).await?;

            let r = fs.relativize_path(tmp_path.path().join(filepath.as_utf8_path()))?;
            let FilesystemPath::FilePath(computed) = r else {
                unreachable!()
            };
            assert_eq!(computed.as_str(), filepath.as_str());
        }

        {
            let dirpath = DirectoryPathBuf::from_str("a/path/to")?;
            fs.create_dir_all(&dirpath).await?;

            let r = fs.relativize_path(tmp_path.path().join(dirpath.as_utf8_path()))?;
            let FilesystemPath::DirectoryPath(computed) = r else {
                unreachable!()
            };
            assert_eq!(computed.as_str(), dirpath.as_str());
        }

        Ok(())
    }
}
