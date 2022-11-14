use clap::Args;

use path_clean::PathClean;
use pcloud_sdk_desktop::storage;

use std::env;
use std::path::Path;
use std::path::PathBuf;
use tracing::{debug, info};

// TODO: Args 'userid' and 'auth' are mutually exclusive, but one of them is always required

#[derive(Args, Debug)]
pub struct InitParams {
    /// Where to run this command, if directory doesn't exist, it will be created
    directory: Option<PathBuf>,

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

pub fn handle(home: &Path, params: &InitParams) {
    let working_dir = (match &params.directory {
        Some(d) => {
            if d.is_absolute() {
                d.to_path_buf()
            } else {
                env::current_dir()
                    .expect("Cannot return current dir")
                    .join(d)
            }
        }
        None => env::current_dir().expect("Cannot return current directory"),
    })
    .clean();
    debug!("Init pCloud folder {}", working_dir.display());

    // TODO: Check if folder is already a pCloud folder

    // Check if we need to authorize or just search for configuration
    let apps_file_path = storage::apps::AppsFile::path(home);
    if params.auth {
        let file_data = storage::apps::AppsFile::write(&apps_file_path);
        let app = file_data
            .content
            .find(&params.client_id)
            .expect("Application not found");

        // TODO: Implement here, factorize this functionality (repeated in auth command)
        info!("Run auth process for app {:#?}", app);
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
        let mut config_data = storage::config::ConfigFile::write(&config_file_path);
        let config = &mut config_data.content.data;
        config.auth.client_id = params.client_id.clone();
        config.auth.userid = token.userid;

        // Create the .pcloudignore file
        let ignore_files_path = storage::ignore_files::IgnoreFiles::path(&working_dir);
        storage::ignore_files::IgnoreFiles::read(&ignore_files_path);
    }
}
