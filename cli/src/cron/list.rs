use std::path::Path;

use tracing::debug;

use pcloud_sync::storage;

use crate::output;

pub fn handle(home: &Path) {
    debug!("List applications from {}", home.display());

    let directories_file_path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::read(&directories_file_path);

    // TODO: Depending on verbosity level... maybe add formatters
    for cron in lock.content.data.directories.iter() {
        let config_file_path = storage::config::ConfigFile::path(&cron.path());
        let lock = storage::config::ConfigFile::read(&config_file_path);
        let config = &lock.content.data;

        println!("{}", cron.path().display());
        output::project::_cron_details(config, cron);
    }
}
