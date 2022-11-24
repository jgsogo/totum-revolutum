use std::path::Path;

use anyhow::{bail, Result};
use clap::Args;
use tracing::debug;

use pcloud_sdk_desktop::run;
use pcloud_sdk_desktop::storage::is_pcloud_dir;

use crate::common::current_wdir;
use crate::common::DirectoryArg;
use crate::errors::CLIErrors;

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

pub async fn handle(home: &Path, params: &RunParams) -> Result<()> {
    let run_command = match params.directory.get_directory_param_abs() {
        Some(p) => {
            debug!("run on directory '{}'.", p.display());
            if !p.exists() {
                bail!(CLIErrors::ExitFailure(format!(
                    "Provided directory doesn't exists: '{}'.",
                    p.display()
                )));
            }
            if let Ok(p) = is_pcloud_dir(&p) {
                run::RunCommand::Directory(p)
            } else {
                bail!(CLIErrors::ExitFailure(format!(
                    "Provided directory is not a ploud-dir: '{}'.",
                    p.display()
                )));
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
    run::handle(home, run_command).await
}
