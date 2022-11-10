use clap::Args;

use crate::data::apps;
use std::path::Path;
use tracing::debug;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(home: &Path, _params: &ListParams) {
    debug!("List applications from {}", home.display());
    let file_data = apps::FileData::read(home);

    // TODO: Depending on verbosity level...
    println!("{:#?}", file_data.content);
}
