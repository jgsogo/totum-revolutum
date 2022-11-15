pub mod apps;
pub mod config;
pub mod cron;
pub mod ignore_files;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

const INSIDE_PROJECT_DIRECTORY: &str = ".pcloud";

/// Check if the given directory is a _pcloud_ directory. It will
/// go back in the directory tree looking for the [`config::ConfigFile`]
///
/// Returns the [PathBuf] to the _pcloud_ directory root if it's a valid path.
pub fn is_pcloud_dir(path: &Path) -> Result<PathBuf> {
    assert!(path.is_absolute(), "Provide absolute path");
    if !path.exists() {
        Err(Error::new(ErrorKind::NotFound, "Not a valid path"))
    } else {
        let config_path = config::ConfigFile::path(path);
        if !config_path.exists() {
            path.parent().map_or(
                Err(Error::new(
                    ErrorKind::NotFound,
                    "pcloud directory not found",
                )),
                is_pcloud_dir,
            )
        } else {
            Ok(path.to_path_buf())
        }
    }
}
