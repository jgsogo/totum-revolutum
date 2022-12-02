use std::path::{Path, PathBuf};

use anyhow::Result;
use flume::Sender;
use ignore::WalkBuilder;
use tokio::time::Instant;
use tracing::info;

use crate::diff::Filesystem;
use crate::local::LocalMetadata;
use crate::storage::ignore_files;

use super::parallel_visitor;

pub struct FilesystemLocal {
    path: PathBuf,
}

impl FilesystemLocal {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    pub async fn walk_directory(&self, tx: flume::Sender<LocalMetadata>, threads: usize) -> Result<()> {
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
}

impl Filesystem for FilesystemLocal {
    type Metadata = LocalMetadata;
}
