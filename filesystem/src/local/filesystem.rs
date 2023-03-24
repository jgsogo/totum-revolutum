use std::fs;
use std::path::StripPrefixError;

use anyhow::{anyhow, bail, Result};
use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use ignore::WalkBuilder;
use tokio::time::Instant;
use tracing::info;

use path_utils::{normalize_path, to_absolute_path};

use crate::local::{LocalMetadata, LocalPath};
use crate::{File, Filesystem};

use super::file::LocalFile;
use super::parallel_visitor;

pub struct FilesystemLocal {
    path: LocalPath,
}

impl FilesystemLocal {
    pub fn new(path: &Utf8Path) -> Result<Self> {
        let path = LocalPath::new_root(path);
        if let Ok(true) = path.try_exists() {
            Ok(Self { path })
        } else {
            Err(anyhow!(
                "Check input path {path}. Cannot use it as root for a filesystem"
            ))
        }
    }
}

#[async_trait]
impl Filesystem for FilesystemLocal {
    type Metadata = LocalMetadata;

    type FilesystemPath = LocalPath;

    /// Converts the input `path` into a relative path that joined with the local root gives
    /// the absolute path to the local file
    fn to_filesystem_path(&self, path: &Utf8Path) -> Result<Self::FilesystemPath> {
        LocalPath::try_from(path, &self.path)
    }

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        let walker = WalkBuilder::new(&self.path)
            .threads(threads)
            .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
            .add_custom_ignore_filename(custom_ignore_filename)
            .build_parallel();

        info!("Start local visitor");
        let start = Instant::now();
        let mut builder = parallel_visitor::VisitorBuilder::new(tx);
        walker.visit(&mut builder);
        info!("Finished local visitor in {:?}", start.elapsed());
        Ok(())
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        full_path.try_exists()
    }

    async fn create(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        let f = AsyncFile::create(&full_path).await?;
        Ok(Box::new(LocalFile::new(f)))
    }

    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        let f = AsyncFile::open(&full_path).await?;
        Ok(Box::new(LocalFile::new(f)))
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        fs::create_dir_all(&full_path).map_err(|e| anyhow!("Error creating the directory: {e}"))
    }

    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        fs::remove_file(&full_path).map_err(|e| anyhow!("Error removing a file: {e}"))
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        fs::remove_dir(&full_path).map_err(|e| anyhow!("Error removing a directory: {e}"))
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let full_path = {
            let path = self.to_filesystem_path(path)?;
            self.path.join(&path)?
        };
        fs::remove_dir_all(&full_path).map_err(|e| anyhow!("Error removing a directory: {e}"))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_root_not_exists() {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let r = FilesystemLocal::new(&utf8_path.join("not-exist"));
        assert!(r.is_err());
    }

    #[test]
    fn test_root() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let fs = FilesystemLocal::new(utf8_path)?;
        // Root is not cannonicalized, it fails in MacOS where tmp directories are inside sym folder
        // #[cfg(target_os = "macos")]
        // assert_ne!(fs::canonicalize(tmp_dir.path())?, fs.root());
        // assert_eq!(tmp_dir.path(), fs.root());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_write_read() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let fs = FilesystemLocal::new(utf8_path)?;

        let filepath = utf8_path.join("myfile");
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Create and write
        {
            let mut f = fs.create(&filepath).await?;
            f.write_all(&content).await?;
        }

        // Open and read
        {
            let mut file = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(content, content_read);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_in_subfolder() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let fs = FilesystemLocal::new(utf8_path)?;

        let filepath = utf8_path.join("nested/nested2/myfile.txt");
        let r = fs.create(&filepath).await;
        assert!(r.is_err());

        fs.create_dir_all(&Utf8PathBuf::from("nested/nested2")).await?;
        let r = fs.create(&filepath).await;
        assert!(r.is_ok());
        Ok(())
    }
}
