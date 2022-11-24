use std::path::Path;

use clap::Args;
use tracing::debug;

use pcloud_sdk_desktop::storage;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(home: &Path, _params: &ListParams) {
    debug!("List applications from {}", home.display());
    let path = storage::apps::AppsFile::path(home);
    let file_data = storage::apps::AppsFile::read(&path);

    // TODO: Depending on verbosity level... maybe add formatters
    println!("{:#?}", file_data.content.data);
}
