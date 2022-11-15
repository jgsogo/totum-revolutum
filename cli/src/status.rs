use clap::Args;

use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::utils::to_absolute_path;

use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

#[derive(Args, Debug)]
pub struct StatusParams {
    /// Where to run this command, if directory doesn't exist, it will be created
    directory: Option<PathBuf>,
}

pub fn handle(_home: &Path, params: &StatusParams) {
    let working_dir = match &params.directory {
        Some(d) => to_absolute_path(d),
        None => to_absolute_path(Path::new(".")),
    };
    debug!("Status pCloud folder {}", working_dir.display());

    // TODO: Check if folder is already a pCloud folder

    let path = storage::config::ConfigFile::path(&working_dir);
    let config_data = storage::config::ConfigFile::read(&path);
    let config = &config_data.content.data;
    println!("client_id: {}", config.auth.client_id);
    println!("userid: {}", config.auth.userid);
    // TODO: println!("action: {}", config.action.action);
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
