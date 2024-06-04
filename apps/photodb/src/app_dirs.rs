use anyhow::{anyhow, Result};
use camino::Utf8PathBuf;
use filesystem::local_temp::FilesystemLocalTemp;
use std::fmt::{Display, Formatter};

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
    pub fn temp_filename(&self, prefix: Option<&str>, suffix: Option<&str>) -> Utf8PathBuf {
        self.local_tmp_storage.temp_filename(prefix, suffix)
    }
}

impl Display for AppDirs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.app_dir)
    }
}
