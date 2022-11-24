use std::path::Path;

use anyhow::Result;

use pcloud_sdk_desktop::storage;

use crate::common::DirectoryArg;
use crate::output;

pub type StatusParams = DirectoryArg;

pub fn handle(home: &Path, params: &StatusParams) -> Result<()> {
    let path = params.get_pcloud_dir();

    let config_file_path = storage::config::ConfigFile::path(&path);
    let lock = storage::config::ConfigFile::try_read(&config_file_path)?;
    let config = &lock.content.data;
    output::project::_project_details(config);

    let directories_file_path = storage::cron::DirectoriesFile::path(home);
    let lock = storage::cron::DirectoriesFile::try_read(&directories_file_path)?;
    if let Some(found) = lock.content.find(&path) {
        output::project::_cron_details(config, found)
    };
    Ok(())
}
