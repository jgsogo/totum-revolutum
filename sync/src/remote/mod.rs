use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use tracing::{info, trace};

pub use file_metadata::RemoteMetadata;
use pcloud_sdk::{
    methods::folder::{listfolder::GetListFolder, ListFolderInput},
    structures::Metadata,
};

use crate::remote::file_metadata::RemoteFileMetadata;
use crate::{diff::basepoint::BasePointDiffImpl, storage::config};

mod file_metadata;

pub async fn walk_remote_directory(
    config: &config::Config,
    _threads: usize,
    diff: BasePointDiffImpl<RemoteMetadata>,
) -> Result<()> {
    let pcloud = config.pcloud.as_ref().unwrap();

    // FIXME: Here we can implement two different strategies. One of them is to iterate everything
    //  from the ROOT folder recursively, the other one is to list the files in each directory
    //  and use a thread pool to enter child directories and _recurse_.
    info!("Start remote visitor");
    let start = Instant::now();
    let mut list_folder_input = ListFolderInput::new_from_path(config.auth.remote_path.clone());
    list_folder_input.recursive = true;
    let filtermeta = vec!["name", "contents", "size", "hash"];
    let items = pcloud
        .listfolder_with_filtermeta(&list_folder_input, filtermeta)
        .await
        .unwrap();

    match &items.metadata.contents {
        Some(contents) => work_on_contents(Path::new(""), contents, &diff, 0)?,
        None => (),
    }
    info!("Finished remote visitor in {:?}", start.elapsed());

    Ok(())
}

fn work_on_contents(
    base_path: &Path,
    contents: &Vec<Metadata>,
    diff: &BasePointDiffImpl<RemoteMetadata>,
    depth: usize,
) -> Result<()> {
    let prefix = format!("{}{}", " ".repeat(depth * 4), PRINT_FOLDER_TOKEN);
    for it in contents.iter() {
        let path = base_path.join(Path::new(it.common.name.as_ref().unwrap()));
        trace!("{}{}", prefix, path.display());
        diff.file_found(RemoteMetadata::from_pcloud_metadata(&*path, it.clone()));
        match &it.contents {
            Some(contents) => work_on_contents(&path, contents, diff, depth + 1)?,
            None => (),
        }
    }
    Ok(())
}

// TODO: This is not the place for outputters
const PRINT_FOLDER_TOKEN: &str = "|-- ";
