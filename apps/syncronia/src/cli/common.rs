use camino::Utf8PathBuf;
use std::env;

use clap::Args;

use syncronia::storage::is_pcloud_dir;
use syncronia::utils::to_absolute_path;

pub fn current_wdir() -> Utf8PathBuf {
    let current_wdir = env::current_dir().expect("Cannot return current dir");
    let path = Utf8PathBuf::from_path_buf(current_wdir).unwrap();
    to_absolute_path(&path)
}

#[derive(Args, Debug)]
pub struct DirectoryArg {
    /// Directory to an existing pcloud configured project
    directory: Option<Utf8PathBuf>,
}

impl DirectoryArg {
    pub fn get_directory_param_abs(&self) -> Option<Utf8PathBuf> {
        self.directory.as_ref().map(|d| to_absolute_path(d))
    }

    pub fn get_working_dir_from_directory_param(&self) -> Utf8PathBuf {
        match self.get_directory_param_abs() {
            Some(d) => d,
            None => current_wdir(),
        }
    }

    pub fn get_pcloud_dir(&self) -> Utf8PathBuf {
        let wdir = self.get_working_dir_from_directory_param();

        match is_pcloud_dir(&wdir) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Provided directory is not a pcloud one: '{wdir}'. {e}");
                std::process::exit(1);
            }
        }
    }
}
