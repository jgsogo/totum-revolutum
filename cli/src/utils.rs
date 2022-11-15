use pcloud_sdk_desktop::storage::is_pcloud_dir;
use pcloud_sdk_desktop::utils::to_absolute_path;
use std::env;
use std::path::PathBuf;

pub trait ParamsOptionalDirectory {
    fn get_directory_param(&self) -> Option<PathBuf>;

    fn get_directory_param_abs(&self) -> Option<PathBuf> {
        self.get_directory_param().map(|d| to_absolute_path(&d))
    }

    fn get_working_dir_from_directory_param(&self) -> PathBuf {
        match self.get_directory_param_abs() {
            Some(d) => d,
            None => {
                let current_wdir = env::current_dir().expect("Cannot return current dir");
                to_absolute_path(&current_wdir)
            }
        }
    }

    fn get_pcloud_dir(&self) -> PathBuf {
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
