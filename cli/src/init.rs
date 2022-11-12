use clap::Args;

use path_clean::PathClean;
use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::LockedFileTrait;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use tracing::{debug, info};

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
    if params.auth {
        let file_data = storage::apps::AppsData::write(home);
        let app = file_data.find(&params.client_id);

        info!("Run auth process for app {:#?}", app);
    } else {
        let file_data = storage::apps::AppsData::read(home);
        let app = file_data
            .find(&params.client_id)
            .expect("Application not found");
        let token = app
            .find_token(params.userid.unwrap())
            .expect("User is not authenticated for the given application");

        let mut config_data = storage::config::ConfigData::write(&working_dir);
        let mut config = config_data.config_as_mut();
        config.client_id = params.client_id.clone();
        config.userid = token.userid;
    }
}
