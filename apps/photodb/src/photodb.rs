use super::db::Database;
use super::utils::sha256_string_from_file;
use anyhow::Result;
use camino::Utf8PathBuf;
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::types::RemotePath;
use tracing::{debug, info};

#[allow(dead_code)]
pub struct PhotoDB<T: Database, PCloud: PCloudClient> {
    pcloud: PCloud,
    db: T,
    app_dir: Utf8PathBuf,
    remote_dir: RemotePath,
}

impl<T: Database, PCloud: PCloudClient> PhotoDB<T, PCloud> {
    pub fn new(db: T, pcloud: PCloud, app_dir: Utf8PathBuf, remote_dir: RemotePath) -> Self {
        info!(
            "New photodb application using local directory '{}' and remote directory '{}'",
            app_dir, remote_dir
        );
        Self {
            pcloud,
            db,
            app_dir,
            remote_dir,
        }
    }

    pub fn add(&self, photo_filepath: Utf8PathBuf) -> Result<()> {
        debug!("Add photo at '{}'", photo_filepath);
        let sha256 = sha256_string_from_file(photo_filepath.as_path())?;
        debug!(" - sha256 '{}'", sha256);
        Ok(())
    }
}
