use crate::errors::SDKErrors;
use crate::storage::config;
use crate::storage::ignore_files;
use anyhow::{anyhow, Result};
use ignore::WalkBuilder;
use std::path::Path;
use tracing::info;

pub fn run(path: &Path, _config: &config::Config) -> Result<()> {
    info!("Run backup action on path '{}'", path.display());

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
