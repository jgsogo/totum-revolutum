use pcloud_sdk_desktop::storage;
use std::path::Path;
use tracing::debug;

pub fn handle(home: &Path) {
    debug!("List applications from {}", home.display());
    let path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::read(&path);

    // TODO: Depending on verbosity level... maybe add formatters
    println!("{:#?}", lock.content.data);
}
