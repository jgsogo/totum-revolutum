use pcloud_sdk_desktop::storage;
use std::path::Path;
use tracing::debug;

pub fn handle(home: &Path) {
    debug!("List applications from {}", home.display());
    let path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::read(&path);

    // TODO: Depending on verbosity level... maybe add formatters
    for cron in lock.content.data.directories.iter() {
        println!("{}", cron.path().display());
        if let Some(upcoming) = cron.upcoming() {
            println!("   next: {}", upcoming);
        }
        // Check last run
        let config_file_path = storage::config::ConfigFile::path(&cron.path());
        let lock = storage::config::ConfigFile::read(&config_file_path);
        let last_executed = match lock.content.data.action.last_executed {
            Some(d) => {
                d.to_string()
            }
            None => "NEVER".into(),
        };
        println!("   last: {}", last_executed);
    }
}
