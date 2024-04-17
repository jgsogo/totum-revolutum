use camino::Utf8Path;

use anyhow::Result;
use tracing::debug;

use syncronia::storage;

use crate::output;

pub fn handle(home: &Utf8Path) -> Result<()> {
    debug!("List applications from {home}");

    let directories_file_path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::read(&directories_file_path)?;

    // TODO: Depending on verbosity level... maybe add formatters
    for cron in lock.content.data.directories.iter() {
        let config_file_path = storage::config::ConfigFile::path(&cron.path());
        let lock = storage::config::ConfigFile::read(&config_file_path)?;
        let config = &lock.content.data;

        println!("{}", cron.path());
        output::project::_cron_details(config, cron);
    }
    Ok(())
}
