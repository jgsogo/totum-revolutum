use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use flume::Sender;
use ignore::WalkBuilder;
use tokio::time::Instant;
use tracing::info;

use crate::diff::{File, Filesystem};
use crate::local::LocalMetadata;
use crate::storage::ignore_files;

use super::file::LocalFile;
use super::parallel_visitor;

pub struct FilesystemLocal {
    path: PathBuf,
}

impl FilesystemLocal {
    pub fn new(path: &Path) -> Result<Self> {
        // We use `fs::canonicalize` to check that the path exists and resolve any symlinks
        let path = fs::canonicalize(path)?;
        Ok(Self { path })
    }
}

#[async_trait]
impl Filesystem for FilesystemLocal {
    type Metadata = LocalMetadata;

    fn root(&self) -> &Path {
        &self.path
    }

    async fn walk_directory(&self, tx: Sender<Self::Metadata>, threads: usize) -> Result<()> {
        let walker = WalkBuilder::new(&self.path)
            .threads(threads)
            .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
            .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(&self.path))
            .build_parallel();

        info!("Start local visitor");
        let start = Instant::now();
        let mut builder = parallel_visitor::VisitorBuilder::new(tx);
        walker.visit(&mut builder);
        info!("Finished local visitor in {:?}", start.elapsed());
        Ok(())
    }

    async fn create(&self, path: &Path) -> Result<Box<dyn File>> {
        // Parent directory should exist
        let parent = path
            .parent()
            .ok_or(anyhow!("Cannot get parent directory for file '{}'", path.display()))?;
        self.check_path(&fs::canonicalize(parent)?)?;

        let f = AsyncFile::create(path).await?;
        Ok(Box::new(LocalFile::new(f)))
    }

    async fn open(&self, path: &Path) -> Result<Box<dyn File>> {
        match self.check_path(&fs::canonicalize(path)?) {
            Ok(v) => {
                let f = AsyncFile::open(v).await?;
                Ok(Box::new(LocalFile::new(f)))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_root_not_exists() {
        let tmp_dir = tempdir().unwrap();
        let r = FilesystemLocal::new(&tmp_dir.path().join("not-exist"));
        assert!(r.is_err());
    }

    #[test]
    fn test_root() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let fs = FilesystemLocal::new(tmp_dir.path())?;
        assert_eq!(fs::canonicalize(tmp_dir.path())?, fs.root());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_write_read() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let fs = FilesystemLocal::new(tmp_dir.path())?;

        let filepath = tmp_dir.path().join("myfile");
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
}
