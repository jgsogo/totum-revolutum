use clap::Args;

use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::storage::is_pcloud_dir;
use pcloud_sdk_desktop::utils::to_absolute_path;

use std::path::Path;
use std::path::PathBuf;
use tracing::{debug, info};

use crate::common::current_wdir;
use crate::common::DirectoryArg;

// TODO: Args 'userid' and 'auth' are mutually exclusive, but one of them is always required

#[derive(Args, Debug)]
pub struct RunParams {
    #[clap(flatten)]
    directory: DirectoryArg,

    /// Application to use
    #[clap(long)]
    filter_client_id: Option<String>,

    /// Application to use
    #[clap(long)]
    filter_user_id: Option<String>,
}

#[derive(Debug)]
enum RunCommand {
    Global,
    Directory(PathBuf),
}

pub fn handle(home: &Path, params: &RunParams) {
    let runCommand = match params.directory.get_directory_param_abs() {
        Some(p) => {
            debug!("run on directory '{}'.", p.display());
            if !p.exists() {
                eprintln!("Provided directory doesn't exists: '{}'.", p.display());
                std::process::exit(1);
            }
            if let Ok(p) = is_pcloud_dir(&p) {
                RunCommand::Directory(p)
            } else {
                eprintln!("Provided directory is not a ploud-dir: '{}'.", p.display());
                std::process::exit(1);
            }
        }
        None => {
            let wdir = current_wdir();
            if let Ok(p) = is_pcloud_dir(&wdir) {
                RunCommand::Directory(p)
            } else {
                RunCommand::Global
            }
        }
    };
    debug!("Run command on {:?}", runCommand);
}
