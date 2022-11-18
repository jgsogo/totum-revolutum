use crate::actions;
use crate::errors::SDKErrors;
use crate::storage;
use anyhow::{anyhow, bail, Result};
use std::path::Path;
use tracing::info;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub fn handle(_home: &Path, path: &Path) -> Result<()> {
    info!("Start project run for path '{}'", path.display());

    let config_file_path = storage::config::ConfigFile::path(path);
    let lock = storage::config::ConfigFile::write(&config_file_path);
    if lock.is_err() {
        bail!(SDKErrors::ProjectLocked(path.to_string_lossy().to_string()));
    }
    let lock = lock.unwrap();

    match lock.content.data.action.action {
        actions::Actions::Backup => {
            actions::backup::run(path, &lock.content.data)?;
            // TODO: Update last-execution time
            Err(anyhow!(SDKErrors::NotImplemented))
        }
        actions::Actions::ZipBackup => Err(anyhow!(SDKErrors::NotImplemented)),
        actions::Actions::Sync => Err(anyhow!(SDKErrors::NotImplemented)),
        actions::Actions::Dump => Err(anyhow!(SDKErrors::NotImplemented)),
    }
}
