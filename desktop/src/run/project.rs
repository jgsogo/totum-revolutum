use std::path::Path;
use tracing::{error, info};

use crate::actions;
use crate::storage;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub fn handle(_home: &Path, path: &Path) {
    info!("Start project run for path '{}'", path.display());

    let config_file_path = storage::config::ConfigFile::path(path);
    let lock = storage::config::ConfigFile::write(&config_file_path);
    if lock.is_err() {
        error!("Lock is already taken. Please, try again after current work finishes");
        return;
    }
    let lock = lock.unwrap();

    match lock.content.data.action.action {
        actions::Actions::Backup => {
            actions::backup::run(path, &lock.content.data);
            // TODO: Update last-execution time
        }
    }
}
