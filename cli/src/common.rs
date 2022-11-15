use clap::Args;

use std::path::PathBuf;

use pcloud_sdk_desktop::storage::is_pcloud_dir;
use pcloud_sdk_desktop::utils::to_absolute_path;
use std::env;

#[derive(Args, Debug)]
pub struct DirectoryArg {
    /// Directory to an existing pcloud configured project
    directory: Option<PathBuf>,
}

impl DirectoryArg {
    pub fn get_directory_param_abs(&self) -> Option<PathBuf> {
        self.directory.as_ref().map(|d| to_absolute_path(d))
    }

    pub fn get_working_dir_from_directory_param(&self) -> PathBuf {
        match self.get_directory_param_abs() {
            Some(d) => d,
            None => {
                let current_wdir = env::current_dir().expect("Cannot return current dir");
                to_absolute_path(&current_wdir)
            }
        }
    }

    pub fn get_pcloud_dir(&self) -> PathBuf {
        let wdir = self.get_working_dir_from_directory_param();

        match is_pcloud_dir(&wdir) {
            Ok(p) => p,
            Err(e) => {
                eprintln!(
                    "Provided directory is not a pcloud one: '{}'. {}",
                    wdir.display(),
                    e
                );
                std::process::exit(1);
            }
        }
    }
}
