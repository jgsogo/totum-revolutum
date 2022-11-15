use pcloud_sdk_desktop::storage;
use std::path::Path;

use crate::common::DirectoryArg;

pub type StatusParams = DirectoryArg;

pub fn handle(home: &Path, params: &StatusParams) {
    let path = params.get_pcloud_dir();

    {
        // Show stats contained within the project folder
        let config_file_path = storage::config::ConfigFile::path(&path);
        let lock = storage::config::ConfigFile::read(&config_file_path);
        let config = &lock.content.data;
        println!("client_id: {}", config.auth.client_id);

        // TODO: Translate client_id to application name

        println!("userid: {}", config.auth.userid);
        println!("action: {:#?}", config.action.action);
        println!(
            "last_executed: {}",
            config
                .action
                .last_executed
                .map_or("NEVER".to_string(), |l| {
                    chrono::DateTime::<chrono::Local>::from(l).to_string()
                })
        );
        // TODO: Show stats about files in this folder
    }

    {
        // Show stats from global pcloud
        let directories_file_path = storage::cron::DirectoriesFile::path(home);
        let lock = storage::cron::DirectoriesFile::read(&directories_file_path);
        if let Some(found) = lock.content.find(&path) {
            println!("{:#?}", found);
        }
    }
}
