use clap::Args;

use pcloud_sdk_desktop::run;
use pcloud_sdk_desktop::storage::is_pcloud_dir;

use std::path::Path;
use tracing::debug;

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

pub fn handle(home: &Path, params: &RunParams) {
    let run_command = match params.directory.get_directory_param_abs() {
        Some(p) => {
            debug!("run on directory '{}'.", p.display());
            if !p.exists() {
                eprintln!("Provided directory doesn't exists: '{}'.", p.display());
                std::process::exit(1);
            }
            if let Ok(p) = is_pcloud_dir(&p) {
                run::RunCommand::Directory(p)
            } else {
                eprintln!("Provided directory is not a ploud-dir: '{}'.", p.display());
                std::process::exit(1);
            }
        }
        None => {
            let wdir = current_wdir();
            if let Ok(p) = is_pcloud_dir(&wdir) {
                run::RunCommand::Directory(p)
            } else {
                run::RunCommand::Global
            }
        }
    };
    debug!("Run command on {:?}", run_command);
    run::handle(home, run_command);
}
