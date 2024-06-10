use std::fs;

use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use flume::Sender;
use ignore::WalkBuilder;
use tokio::sync::oneshot::Receiver;
use tokio::time::Instant;
use tracing::info;

use crate::filesystem_async_drop::FilesystemAsyncDrop;
use crate::{Error, Result};
use crate::{File, Filesystem};

use super::file_metadata::LocalMetadata;
use super::parallel_visitor;

/// Implementation of [`Filesystem`] using a directory in the host filesystem.
#[derive(Debug)]
pub struct FilesystemLocal {
    root: Utf8PathBuf,
    fs_async_drop: FilesystemAsyncDrop<AsyncFile>,
}

impl FilesystemLocal {
    /// Creates a new [`FilesystemLocal`] at the given `root` path.
    pub fn new(root: &Utf8Path) -> Result<Self> {
        if !root.exists() {
            return Err(Error::PathDoesNotExist);
        }

        let fs_async_drop = FilesystemAsyncDrop::new(|async_file: AsyncFile| async move {
            async_file.sync_all().await.map_err(Error::IoError)
        });

        Ok(Self {
            root: root.to_path_buf(),
            fs_async_drop,
        })
    }
}

#[async_trait]
impl Filesystem for FilesystemLocal {
    type Metadata = LocalMetadata;

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        threads: usize,
        custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        let walker = WalkBuilder::new(&self.root)
            .threads(threads)
            .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
            .add_custom_ignore_filename(custom_ignore_filename)
            .build_parallel();

        info!("Start local visitor");
        let start = Instant::now();
        let mut builder = parallel_visitor::VisitorBuilder::new(tx, self.root.clone());
        walker.visit(&mut builder);
        info!("Finished local visitor in {:?}", start.elapsed());
        Ok(())
    }

    async fn get_metadata(&self, _path: &Utf8Path) -> Result<Self::Metadata> {
        // It doesn't make much sense that the `Self::Metadata` contains an `ignore::DirEntry`, we
        // need something more identificable as metadata in a local filesystem
        todo!("Not implemented")
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        let path = self.root.join(self.check_path(path)?);
        Ok(path.exists())
    }

    async fn create(&self, path: &Utf8Path) -> Result<(Box<dyn File>, Receiver<Result<()>>)> {
        let path = self.root.join(self.check_path(path)?);
        let f = AsyncFile::create(path.into_std_path_buf()).await?;
        Ok(self.fs_async_drop.file_wrapped(f))
    }

    async fn open(&self, path: &Utf8Path) -> Result<Box<dyn File>> {
        let path = self.root.join(self.check_path(path)?);
        let f = AsyncFile::open(path.into_std_path_buf()).await?;
        // We can drop the receiver here, the file is opened in read-only mode so there is nothing
        // to sync when it is dropped
        let (file, _) = self.fs_async_drop.file_wrapped(f);
        Ok(file)
    }

    async fn create_dir_all(&self, path: &Utf8Path) -> Result<()> {
        let path = self.root.join(self.check_path(path)?);
        fs::create_dir_all(path).map_err(Error::IoError)
    }

    async fn remove_file(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(self.check_path(path)?);
        fs::remove_file(path).map_err(Error::IoError)
    }

    async fn remove_dir(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(self.check_path(path)?);
        fs::remove_dir(path).map_err(Error::IoError)
    }

    async fn remove_dir_all(&self, path: &Utf8Path) -> Result<()> {
        // Do not resolve symlinks
        let path = self.root.join(self.check_path(path)?);
        fs::remove_dir_all(path).map_err(Error::IoError)
    }

    async fn sync_all(mut self) -> Result<()> {
        self.fs_async_drop.flush().await
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_root_not_exists() {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let r = FilesystemLocal::new(&utf8_path.join("not-exist"));
        assert!(r.is_err());
        assert!(matches!(r.unwrap_err(), Error::PathDoesNotExist))
    }

    #[tokio::test]
    async fn test_root() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let utf8_path = Utf8Path::from_path(tmp_dir.path()).unwrap();
        let fs = FilesystemLocal::new(utf8_path)?;
        // Root is not canonical, it fails in MacOS where tmp directories are inside sym folder
        #[cfg(target_os = "macos")]
        assert_ne!(fs::canonicalize(tmp_dir.path())?, fs.root);
        assert_eq!(tmp_dir.path(), fs.root);
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
        let rx = {
            let (mut f, rx) = fs.create(&filepath).await?;
            f.write_all(&content).await?;
            rx
        };
        let _ = rx.await;

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

        // I can't create a file in a subfolder (if the folder doesn't exist yet)
        let r = fs.create("nested/nested2/myfile.txt".into()).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::IoError(io::Error { .. })));

        fs.create_dir_all("nested/nested2".into()).await?;
        let r = fs.create("nested/nested2/myfile.txt".into()).await;
        assert!(r.is_ok());
        Ok(())
    }
}
