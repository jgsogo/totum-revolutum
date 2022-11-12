use clap::Args;

use pcloud_sdk_desktop::storage;

use std::path::Path;
use tracing::debug;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(home: &Path, _params: &ListParams) {
    debug!("List applications from {}", home.display());
    let file_data = storage::apps::FileData::read(home);

    // TODO: Depending on verbosity level...
    println!("{:#?}", file_data.apps());
}
