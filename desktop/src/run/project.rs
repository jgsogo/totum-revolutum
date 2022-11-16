use std::path::Path;

use tracing::info;

/// Run configured action in the given pcloud-dir. It doesn't take into account
/// any cron considerations (those are stored at global level)
pub fn handle(home: &Path, path: &Path) {
    info!("Start project run for path '{}'", path.display());
}
