use std::path::Path;

use anyhow::Result;
use tracing::info;

use crate::actions;
use crate::errors::SDKErrors;
use crate::storage;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub async fn handle(home: &Path, path: &Path) -> Result<()> {
    info!("Start project run for path '{}'", path.display());

    let config_file_path = storage::config::ConfigFile::path(path);
    let mut lock = storage::config::ConfigFile::update(&config_file_path)
        .map_err(|_| SDKErrors::ProjectLocked(path.to_string_lossy().to_string()))?;

    let data = &mut lock.content.data;
    actions::run(home, path, data).await
}
