use camino::Utf8Path;

use anyhow::Result;
use clap::Args;
use tracing::debug;

use syncronia::storage;

#[derive(Args, Debug)]
pub struct ListParams {
    name: Option<String>,
}

pub fn handle(home: &Utf8Path, _params: &ListParams) -> Result<()> {
    debug!("List applications from {home}");
    let path = storage::apps::AppsFile::path(home);
    let file_data = storage::apps::AppsFile::read(&path)?;

    // TODO: Depending on verbosity level... maybe add formatters
    println!("{:#?}", file_data.content.data);
    Ok(())
}
