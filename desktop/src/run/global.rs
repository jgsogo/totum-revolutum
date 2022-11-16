use std::path::Path;

use super::super::storage;
use tracing::{debug, info};

/// Run configured action for the directories where cron is configured
/// and time is elapsed
pub fn handle(home: &Path) {
    info!("Start global run");

    let directories_file_path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::read(&directories_file_path);

    let now = chrono::Utc::now();

    for cron in lock.content.data.directories.iter() {
        debug!("Work on path '{}'", cron.path().display());
        let config_file_path = storage::config::ConfigFile::path(&cron.path());
        let lock = storage::config::ConfigFile::read(&config_file_path);
        let config = &lock.content.data;

        match config.action.last_executed {
            None => {
                debug!("No last execution recorded. Triggering right away");
                super::project::handle(home, &cron.path())
            }
            Some(d) => match cron.next(&now) {
                Some(next) => {
                    debug!("Next scheduled execution at {}", next);
                    if d <= now {
                        debug!("Trigger execution! {} <= {}", next, now);
                        super::project::handle(home, &cron.path())
                    }
                }
                None => {
                    debug!("Unexpectedly, there is no next scheduled execution");
                }
            },
        };
    }
}
