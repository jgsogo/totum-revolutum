use clap::Args;

use pcloud_sdk_desktop::storage;
use pcloud_sdk_desktop::LockedFileTrait;
use std::path::Path;
use tracing::debug;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(home: &Path, _params: &ListParams) {
    debug!("List applications from {}", home.display());
    let file_data = storage::apps::AppsData::read(home);

    // TODO: Depending on verbosity level... maybe add formatters
    println!("{:#?}", file_data.apps());
}
