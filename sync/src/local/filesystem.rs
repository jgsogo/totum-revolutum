use std::path::{Path, PathBuf};

use anyhow::Result;
use ignore::WalkBuilder;
use tokio::time::Instant;
use tracing::info;

use crate::diff::filesystem::Filesystem;
use crate::local::{LocalFileMetadata, LocalMetadata};
use crate::storage::ignore_files;

use super::parallel_visitor;

pub struct FilesystemLocal {
    path: PathBuf,
    tx: flume::Sender<LocalMetadata>,
}

impl FilesystemLocal {
    pub fn new(path: &Path, tx: flume::Sender<LocalMetadata>) -> Self {
        Self {
            path: path.to_path_buf(),
            tx,
        }
    }

    pub async fn walk_directory(&self, threads: usize) -> Result<()> {
        let walker = WalkBuilder::new(&self.path)
            .threads(threads)
            .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
            .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(&self.path))
            .build_parallel();

        info!("Start local visitor");
        let start = Instant::now();
        let mut builder = parallel_visitor::VisitorBuilder::new(self);
        walker.visit(&mut builder);
        info!("Finished local visitor in {:?}", start.elapsed());
        Ok(())
    }

    pub fn file_found(&self, entry: ignore::DirEntry) {
        let meta = LocalMetadata::from_direntry(&self.path, entry);
        self.tx.send(meta).expect("TODO: Something to implement");
    }
}

impl Filesystem<LocalMetadata> for FilesystemLocal {}
