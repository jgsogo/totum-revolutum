use crate::errors::SDKErrors;
use crate::storage::config;
use crate::storage::ignore_files;
use anyhow::{anyhow, Result};
use ignore::{WalkBuilder, WalkState};
use std::path::Path;
use tracing::info;

pub fn run(path: &Path, _config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

    let walker = WalkBuilder::new(path)
        .threads(6)
        .git_global(false) // TODO: Disable all ignore files: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L750
        .add_custom_ignore_filename(ignore_files::IgnoreFiles::path(path))
        .build_parallel();

    // TODO: Better to add all PATHS to the same walker than to instantiate a new one for each: https://github.com/BurntSushi/ripgrep/blob/master/crates/ignore/src/walk.rs#L610

    // TODO: Figure out how to use `visit` (https://docs.rs/ignore/latest/ignore/struct.WalkParallel.html#method.visit), we can group together
    //  the files in each iterator to work in batches.

    walker.run(|| {
        Box::new(move |result| {
            let entry: ignore::DirEntry = result.unwrap();
            let meta = entry.metadata().unwrap();
            // println!("{}", entry.path().display());
            // println!(" - is_file: {}", meta.is_file());
            // println!(" - modified: {:?}", meta.modified().unwrap());
            // println!(" - file_type: {:?}", meta.file_type());
            WalkState::Continue
        })
    });
    Ok(())
    // Err(anyhow!(SDKErrors::NotImplemented))
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
