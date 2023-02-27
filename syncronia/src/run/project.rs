use camino::Utf8Path;

use anyhow::Result;
use tracing::info;

use crate::actions;
use crate::errors::SDKErrors;
use crate::storage;
use filesystem::local::FilesystemLocal;
use filesystem_pcloud::FilesystemPCloud;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub async fn handle(home: &Utf8Path, path: &Utf8Path) -> Result<()> {
    info!("Start project run for path '{path}'");

    let config_file_path = storage::config::ConfigFile::path(path);
    let mut lock = storage::config::ConfigFile::update(&config_file_path)
        .map_err(|_| SDKErrors::ProjectLocked(path.to_string()))?;

    let config = &mut lock.content.data;

    // TODO: We cannot assume lhs filesystem is the local one
    let lhs_fs = FilesystemLocal::new(path)?;

    // TODO: We cannot assume rhs filesystem is a remote-pcloud one
    let rhs_fs = {
        let pcloud = config.auth.get_pcloud_client(home)?;
        let base_path = config.auth.remote_path.as_ref().unwrap_or(&"/".to_string()).clone();
        FilesystemPCloud::new(Utf8Path::new(&base_path), pcloud.clone()).await?
    };

    actions::run(lhs_fs, rhs_fs, config).await
}
