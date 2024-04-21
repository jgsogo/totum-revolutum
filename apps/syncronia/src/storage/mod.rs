use camino::{Utf8Path, Utf8PathBuf};
use std::io::{Error, ErrorKind, Result};
use std::result;

pub mod app;
pub mod apps;
pub mod config;
pub mod cron;
pub mod ignore_files;

const INSIDE_PROJECT_DIRECTORY: &str = ".pcloud";

/// Check if the given directory is a _pcloud_ directory. It will
/// go back in the directory tree looking for the [`config::ConfigFile`]
///
/// Returns the [Utf8PathBuf] to the _pcloud_ directory root if it's a valid path.
pub fn is_pcloud_dir(path: &Utf8Path) -> Result<Utf8PathBuf> {
    assert!(path.is_absolute(), "Provide absolute path");
    if !path.exists() {
        Err(Error::new(ErrorKind::NotFound, "Not a valid path"))
    } else {
        let config_path = config::ConfigFile::path(path);
        if !config_path.exists() {
            path.parent().map_or(
                Err(Error::new(ErrorKind::NotFound, "pcloud directory not found")),
                is_pcloud_dir,
            )
        } else {
            Ok(path.to_path_buf())
        }
    }
}

/// Returns if the given `path` is a valid candidate as a pcloud dir
///
/// Valid candidates are any folder that is not already a pcloud-dir
/// or under a valid pcloud-dir (even if the directory doesn't exist yet)
pub fn candidate_pcloud_dir(path: &Utf8Path) -> result::Result<bool, Utf8PathBuf> {
    if is_pcloud_dir(path).is_ok() {
        return Err(path.to_path_buf());
    }
    if path.exists() {
        return Ok(true);
    }

    // Given path doesn't exist, we need to check if any of the parents is already a pcloud-dir
    match path.parent() {
        Some(p) => candidate_pcloud_dir(p),
        None => Err(path.to_path_buf()), // TODO: Better error from here
    }
}
