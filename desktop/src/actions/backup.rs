use super::parallel_visitor;
use crate::changes;
use crate::errors::SDKErrors;
use crate::storage::config;
use crate::storage::ignore_files;
use anyhow::{anyhow, Result};
use ignore::WalkBuilder;
use std::path::Path;
use tracing::info;

pub async fn run(path: &Path, _config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    let walker = WalkBuilder::new(path)
        .threads(6)
        .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
        .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(path))
        .build_parallel();

    let walker2 = WalkBuilder::new(path)
        .threads(6)
        .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
        .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(path))
        .build_parallel();

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610
    let (lhs, rhs, mut report) = changes::two_ways::TwoWaysDiff::<
        changes::local::LocalMetadata,
        changes::local::LocalMetadata,
    >::new();

    let wait_lhs = {
        let path = path.to_path_buf();
        tokio::spawn(async move {
            info!("Start LHS visitor");
            let mut builder = parallel_visitor::VisitorBuilder::new(&path, &lhs);
            walker.visit(&mut builder);
            info!("Finished LHS visitor");
        })
    };

    let wait_rhs = {
        let path = path.to_path_buf();
        tokio::spawn(async move {
            info!("Start RHS visitor");
            let mut builder = parallel_visitor::VisitorBuilder::new(&path, &rhs);
            walker2.visit(&mut builder);
            info!("Finished RHS visitor");
        })
    };

    report.report().await;

    wait_lhs.await?;
    wait_rhs.await?;
    Err(anyhow!(SDKErrors::NotImplemented))
}

#[allow(dead_code)]
pub fn run2(path: &Path, _config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    // TODO: Evaluate WalkParallel
    let mut builder = WalkBuilder::new(path);
    builder.add_custom_ignore_filename(ignore_files::IgnoreFiles::path(path));
    let walker = builder.build();
    for result in walker {
        let entry: ignore::DirEntry = result.unwrap();
        let meta = entry.metadata().unwrap();
        println!("{}", entry.path().display());
        println!(" - is_file: {}", meta.is_file());
        println!(" - modified: {:?}", meta.modified().unwrap());
        println!(" - file_type: {:?}", meta.file_type());
    }

    Err(anyhow!(SDKErrors::NotImplemented))
}
