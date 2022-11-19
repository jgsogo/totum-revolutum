use crate::actions;
use crate::errors::SDKErrors;
use crate::storage;
use anyhow::{anyhow, Result};
use std::path::Path;
use tracing::info;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub fn handle(_home: &Path, path: &Path) -> Result<()> {
    info!("Start project run for path '{}'", path.display());

    let config_file_path = storage::config::ConfigFile::path(path);
    let mut lock = storage::config::ConfigFile::write(&config_file_path)
        .map_err(|_| SDKErrors::ProjectLocked(path.to_string_lossy().to_string()))?;

    let data = &mut lock.content.data;
    match data.action.action {
        actions::Actions::Backup => {
            let now = chrono::Utc::now();

            actions::backup::run(path, data)?;

            // Update last-execution time. We use the timestamp when the process started because files might be modified
            //  while we are running it and after they are synced. We use the `now` we created above!!!
            data.action.last_executed = Some(now);
            Ok(())
        }
        actions::Actions::ZipBackup => Err(anyhow!(SDKErrors::NotImplemented)),
        actions::Actions::Sync => Err(anyhow!(SDKErrors::NotImplemented)),
        actions::Actions::Dump => Err(anyhow!(SDKErrors::NotImplemented)),
    }
}
