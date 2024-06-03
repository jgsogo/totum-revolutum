use camino::Utf8Path;
use std::str::FromStr;

use anyhow::Result;
use clap::Args;

use syncronia::storage;
use syncronia::storage::cron;

use crate::common::DirectoryArg;

#[derive(Args, Debug)]
pub struct AddParams {
    #[clap(flatten)]
    directory: DirectoryArg,

    #[clap(long)]
    /// Expression
    cron_expression: String,

    // TODO: Default to local
    #[clap(long, default_value = "UTC")]
    /// Expression
    cron_tz: String,
}

pub fn handle(home: &Utf8Path, params: &AddParams) -> Result<()> {
    let path = params.directory.get_pcloud_dir();

    let directory_entry = cron::Directory::new(
        &path,
        &params.cron_expression,
        &chrono_tz::Tz::from_str(&params.cron_tz).unwrap(),
    )?;

    let directories_file_path = storage::cron::DirectoriesFile::path(home);
    let mut lock = storage::cron::DirectoriesFile::update(&directories_file_path)?;
    let (entry, inserted) = lock.content.find_or_insert(&path, directory_entry);

    if !inserted {
        eprintln!(
            "There is already an entry for the same path '{}'. Remove it first.",
            entry.path()
        );
        std::process::exit(1);
    }

    println!(
        "Added directory '{}', next run: '{}'",
        entry.path(),
        entry.upcoming().unwrap()
    );
    Ok(())
}
