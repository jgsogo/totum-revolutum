use std::path::{Path, PathBuf};

use super::super::storage;
use tracing::{debug, info, warn};

async fn run_project(home: PathBuf, path: PathBuf) {
    super::project::handle(&home, &path)
}

/// Run configured action for the directories where cron is configured
/// and time is elapsed
pub async fn handle(home: &Path) {
    info!("Start global run");

    // Collect the tasks and execute them asyncronously
    let mut tasks = {
        let mut tasks = tokio::task::JoinSet::new();
        let directories_file_path = storage::cron::DirectoriesFile::path(home);
        let lock = storage::cron::DirectoriesFile::read(&directories_file_path);

        let now = chrono::Utc::now();

        for cron in lock.content.data.directories.iter() {
            debug!("Work on path '{}'", cron.path().display());
            let last_executed = {
                let config_file_path = storage::config::ConfigFile::path(&cron.path());
                match storage::config::ConfigFile::try_read(&config_file_path) {
                    Ok(lock) => lock.content.data.action.last_executed,
                    Err(_) => {
                        warn!("Project is running some blocking operation. Try again later");
                        break;
                    }
                }
            };

            match last_executed {
                None => {
                    debug!("No last execution recorded. Triggering right away");
                    tasks.spawn(run_project(home.to_path_buf(), cron.path()));
                }
                Some(d) => match cron.next(&now) {
                    Some(next) => {
                        debug!("Next scheduled execution at {}", next);
                        if d <= now {
                            debug!("Trigger execution! {} <= {}", next, now);
                            tasks.spawn(run_project(home.to_path_buf(), cron.path()));
                        }
                    }
                    None => {
                        debug!("Unexpectedly, there is no next scheduled execution");
                    }
                },
            };
        }
        tasks
    };

    while let Some(_res) = tasks.join_next().await {
        // let idx = res.unwrap();
    }
}
