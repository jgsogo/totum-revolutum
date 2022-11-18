use crate::storage::config;
use std::path::Path;
use tracing::info;

pub fn run(path: &Path, _config: &config::Config) {
    info!("Run backup action on path '{}'", path.display());

    std::thread::sleep(std::time::Duration::from_secs(5));
    info!("Done backup action on path '{}'", path.display());
}
