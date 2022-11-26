use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use ignore::WalkBuilder;
use tracing::info;

pub use file_metadata::LocalMetadata;

use super::diff::basepoint::BasePointDiffImpl;
use super::storage::ignore_files;

mod file_metadata;
mod parallel_visitor;

pub async fn walk_local_directory(
    path: &Path,
    threads: usize,
    diff: BasePointDiffImpl<file_metadata::LocalMetadata>,
) -> Result<()> {
    let walker = WalkBuilder::new(path)
        .threads(threads)
        .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
        .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(path))
        .build_parallel();

    info!("Start local visitor");
    let start = Instant::now();
    let mut builder = parallel_visitor::VisitorBuilder::new(&path, diff);
    walker.visit(&mut builder);
    info!("Finished local visitor in {:?}", start.elapsed());
    Ok(())
}
