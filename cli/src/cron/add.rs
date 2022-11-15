use clap::Args;
use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::storage::cron;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::utils::ParamsOptionalDirectory;

#[derive(Args, Debug)]
pub struct AddParams {
    /// Where to run this command, if directory doesn't exist, it will be created
    directory: Option<PathBuf>,

    #[clap(long)]
    /// Expression
    cron_expression: String,

    // TODO: Default to local
    #[clap(long, default_value = "UTC")]
    /// Expression
    cron_tz: String,
}

impl ParamsOptionalDirectory for AddParams {
    fn get_directory_param(&self) -> Option<PathBuf> {
        self.directory.clone()
    }
}

pub fn handle(home: &Path, params: &AddParams) {
    let path = params.get_pcloud_dir();

    let directory_entry = cron::Directory::new(
        &path,
        &params.cron_expression,
        &chrono_tz::Tz::from_str(&params.cron_tz).unwrap(),
    );

    let path = storage::cron::DirectoriesFile::path(home);
    let mut lock = storage::cron::DirectoriesFile::write(&path);
    let (entry, inserted) = lock.content.find_or_insert(&path, directory_entry);

    if !inserted {
        eprintln!(
            "There is already an entry for the same path '{}'. Remove it first.",
            entry.path().to_string_lossy()
        );
        std::process::exit(1);
    }

    println!(
        "Added directory '{}', next run: '{}'",
        entry.path().to_string_lossy(),
        entry.upcoming().unwrap()
    );
}
