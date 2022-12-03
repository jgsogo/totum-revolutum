use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use flume::Sender;
use ignore::WalkBuilder;
use tokio::time::Instant;
use tracing::info;

use async_std::fs::File as AsyncFile;

use crate::diff::{File, Filesystem};
use crate::local::LocalMetadata;
use crate::storage::ignore_files;

use super::file::LocalFile;
use super::parallel_visitor;

pub struct FilesystemLocal {
    path: PathBuf,
}

impl FilesystemLocal {
    pub fn new(path: &Path) -> Self {
        // We use `fs::canonicalize` because this path should exist
        let canonical = fs::canonicalize(path).unwrap();
        Self { path: canonical }
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

    async fn open(&self, path: &Path) -> Result<Box<dyn File>> {
        match self.check_path(path) {
            Ok(v) => {
                let f = AsyncFile::open(v).await?;
                Ok(Box::new(LocalFile::new(f)))
            }
            Err(e) => Err(e),
        }
    }
}
