pub mod apps;
pub mod config;
pub mod cron;
pub mod ignore_files;
use std::io::{ErrorKind, Result};
use std::path::Path;

const INSIDE_PROJECT_DIRECTORY: &str = ".pcloud";

/// Check if the given directory is a _pcloud_ directory
pub fn is_pcloud_dir(path: &Path) -> Result<bool> {
    assert!(path.is_absolute(), "Provide absolute path");
    if !path.exists() {
        Err(std::io::Error::new(ErrorKind::NotFound, "Token not found"))
    } else {
        let config_path = config::ConfigFile::path(path);
        Ok(config_path.exists())
    }
}
