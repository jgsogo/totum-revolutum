use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use ignore_files::{IgnoreFile, IgnoreFilter};
use tracing::debug;

use filesystem::{impls::FilesystemLocalTemp, FilePathBuf};

const PCLOUD_TOKEN_FILENAME: &str = ".pcloud";
const IGNORE_FILE: &str = ".ignore_file";

/// Manage all the directories related to a [`super::PhotoDB`] application
pub struct AppDirs {
    app_dir: Utf8PathBuf,
    local_tmp_storage: FilesystemLocalTemp,
}

impl AppDirs {
    /// Return a new [`AppDirs`] instance. This method will check that all the directories exist
    /// and, if they don't, it will try to create them. If it's not possible to create those
    /// directories, it will fail.
    pub fn new(app_dir: Utf8PathBuf) -> Result<Self> {
        let r = Self {
            app_dir,
            local_tmp_storage: FilesystemLocalTemp::default(),
        };
        r.create_all_dirs()?;
        Ok(r)
    }

    /// Returns the path to the root folder of the application directory.
    pub fn root(&self) -> &Utf8Path {
        &self.app_dir
    }

    /// Returns the path to the token file
    pub fn pcloud_token(&self) -> Utf8PathBuf {
        self.app_dir.join(Utf8Path::new(PCLOUD_TOKEN_FILENAME))
    }

    pub async fn ignore_filters(&self) -> Result<IgnoreFilter> {
        let ignore_file = self.root().join(Utf8Path::new(IGNORE_FILE));
        if ignore_file.exists() {
            debug!("Use ignore file from {}", ignore_file);
            let origin = PathBuf::from("");
            let ignore_file = IgnoreFile {
                path: ignore_file.into_std_path_buf(),
                applies_in: Some(origin.clone()),
                applies_to: None,
            };
            Ok(IgnoreFilter::new(".", &[ignore_file]).await?)
        } else {
            Ok(IgnoreFilter::empty(""))
        }
    }

    /// Execute [`std::fs::create_dir_all`] for all the directories related to the application.
    /// It will return an error if any of the calls fails.
    fn create_all_dirs(&self) -> anyhow::Result<()> {
        for d in [self.db_backups(), self.cache()] {
            std::fs::create_dir_all(&d).map_err(|e| anyhow!("Failed to create '{d}': {e}"))?;
        }
        Ok(())
    }

    /// You can find here backups of the DB when it has failed to upload them
    pub fn db_backups(&self) -> Utf8PathBuf {
        self.app_dir.join("db")
    }

    /// A local cache for performance/bandwidth optimization. It's safe to remove
    pub fn cache(&self) -> Utf8PathBuf {
        self.app_dir.join("cache")
    }

    /// Returns a filename that will be removed after the application goes out of scope.
    ///
    /// User can provide a prefix and suffix for the created filename. This method will add some
    /// randomness (uuid4) to the filename so uniqueness can be assumed.
    pub fn temp_filename(&self, prefix: Option<&str>, suffix: Option<&str>) -> FilePathBuf {
        self.local_tmp_storage.temp_filename(prefix, suffix)
    }
}

impl Display for AppDirs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.app_dir)
    }
}
