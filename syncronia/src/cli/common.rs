use std::env;
use std::path::PathBuf;

use clap::Args;

use syncronia::storage::is_pcloud_dir;
use syncronia::utils::to_absolute_path;

pub fn current_wdir() -> PathBuf {
    let current_wdir = env::current_dir().expect("Cannot return current dir");
    to_absolute_path(&current_wdir)
}

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
            None => current_wdir(),
        }
    }

    pub fn get_pcloud_dir(&self) -> PathBuf {
        let wdir = self.get_working_dir_from_directory_param();

        match is_pcloud_dir(&wdir) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Provided directory is not a pcloud one: '{}'. {}", wdir.display(), e);
                std::process::exit(1);
            }
        }
    }
}
