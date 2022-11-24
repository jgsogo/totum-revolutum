mod file_metadata;

use crate::{diff::basepoint::BasePointDiffImpl, storage::config};
use anyhow::Result;
pub use file_metadata::RemoteMetadata;
use pcloud_sdk::{
    methods::folder::{listfolder::GetListFolder, ListFolderInput},
    structures::Metadata,
};

pub async fn walk_remote_directory(
    config: &config::Config,
    _threads: usize,
    _diff: BasePointDiffImpl<file_metadata::RemoteMetadata>,
) -> Result<()> {
    let pcloud = config.pcloud.as_ref().unwrap();

    let mut list_folder_input = ListFolderInput::new_from_path(config.auth.remote_path.clone());
    list_folder_input.recursive = true;
    let filtermeta = vec!["name", "contents", "size", "hash"];
    let items = pcloud
        .listfolder_with_filtermeta(&list_folder_input, filtermeta)
        .await
        .unwrap();

    outputter(&items.metadata, 0);

    Ok(())
}

// TODO: This is not the place for outputters
const PRINT_FOLDER_TOKEN: &str = "|-- ";
fn outputter(metadata: &Metadata, nested: usize) {
    let prefix = format!("{}{}", " ".repeat(nested), PRINT_FOLDER_TOKEN);

    match &metadata.contents {
        Some(contents) => {
            for it in contents.iter() {
                println!(
                    "{}{} ({} - {})",
                    prefix,
                    it.common.name.as_ref().unwrap(),
                    it.size.unwrap_or(0),
                    it.hash.unwrap_or(0),
                );
                outputter(it, nested + 4);
            }
        }
        None => (),
    }
}
