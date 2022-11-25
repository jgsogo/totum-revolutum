use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Args;
use tracing::{debug, info};

use pcloud_sync::storage;
use pcloud_sync::storage::candidate_pcloud_dir;

use crate::common::DirectoryArg;

// TODO: Args 'userid' and 'auth' are mutually exclusive, but one of them is always required

#[derive(Args, Debug)]
pub struct InitParams {
    #[clap(flatten)]
    directory: DirectoryArg,

    /// Application to use within this directory
    #[clap(long)]
    client_id: String,

    /// User
    #[clap(long, group = "user_id")]
    userid: Option<i32>,

    #[clap(long, action, group = "user_id")]
    /// Run authorize for the application
    auth: bool,
}

pub fn handle(home: &Path, params: &InitParams) -> Result<()> {
    let working_dir = params.directory.get_working_dir_from_directory_param();
    debug!("Init pCloud folder {}", working_dir.display());

    if let Err(e) = candidate_pcloud_dir(&working_dir) {
        eprintln!(
            "Provided directory (or one of its parents) is already a pcloud one: '{}'",
            e.display()
        );
        std::process::exit(1);
    }

    // Check if we need to authorize or just search for configuration
    let apps_file_path = storage::apps::AppsFile::path(home);
    if params.auth {
        let file_data = storage::apps::AppsFile::write(&apps_file_path)?;
        let app = file_data
            .content
            .find(&params.client_id)
            .expect("Application not found");

        // TODO: Implement here, factorize this functionality (repeated in auth command)
        info!("Run auth process for app {:#?}", app);
        Err(anyhow!("Not implemented!"))
    } else {
        let file_data = storage::apps::AppsFile::read(&apps_file_path);
        let app = file_data
            .content
            .find(&params.client_id)
            .expect("Application not found");
        let token = app
            .find_token(params.userid.unwrap())
            .expect("User is not authenticated for the given application");

        // Create the .pcloud/config file
        let config_file_path = storage::config::ConfigFile::path(&working_dir);
        let mut config_data = storage::config::ConfigFile::write(&config_file_path)?;
        let config = &mut config_data.content.data;
        config.auth.client_id = params.client_id.clone();
        config.auth.userid = token.userid;

        // Create the .pcloudignore file
        let ignore_files_path = storage::ignore_files::IgnoreFiles::path(&working_dir);
        storage::ignore_files::IgnoreFiles::read(&ignore_files_path);
        Ok(())
    }
}
