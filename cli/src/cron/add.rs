use clap::Args;
use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::storage::{cron, is_pcloud_dir};
use std::{path::Path, str::FromStr};

#[derive(Args, Debug)]
pub struct AddParams {
    #[clap(long)]
    /// Path to the directory
    path: String,

    #[clap(long)]
    /// Expression
    cron_expression: String,

    // TODO: Default to local
    #[clap(long, default_value = "UTC")]
    /// Expression
    cron_tz: String,
}

pub fn handle(home: &Path, params: &AddParams) {
    let directory_entry = cron::Directory::new(
        Path::new(&params.path),
        &params.cron_expression,
        &chrono_tz::Tz::from_str(&params.cron_tz).unwrap(),
    );
    let directory_path = directory_entry.path();
    if !is_pcloud_dir(&directory_path).unwrap_or(false) {
        eprintln!(
            "Provided directory is not a pcloud one: {}",
            directory_path.display()
        );
        std::process::exit(1);
    }

    let path = storage::cron::DirectoriesFile::path(home);
    let mut lock = storage::cron::DirectoriesFile::write(&path);
    let (entry, inserted) = lock
        .content
        .find_or_insert(&directory_path, directory_entry);

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
