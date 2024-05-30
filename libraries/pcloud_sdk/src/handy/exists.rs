use anyhow::Result;
use async_trait::async_trait;

use crate::methods::folder::{listfolder, ListFolderInput};
use crate::structures::Metadata;
use crate::types::{FileID, Folder, FolderID};

#[async_trait]
/// Checks if the given `filename` exists withing the `folder_id`. Returns the [`FileID`] if it exists
pub trait Exists {
    async fn exists(&self, folder_id: FolderID, filename: &str) -> Result<Option<FileID>>;
}

#[async_trait]
impl<T: listfolder::GetListFolder + Sync> Exists for T {
    async fn exists(&self, folder_id: FolderID, filename: &str) -> Result<Option<FileID>> {
        // Search the folder content for the file we are looking for
        let folder_content = self
            .listfolder_with_filtermeta(ListFolderInput::new(Folder::from(folder_id)), vec!["fileid", "name"])
            .await?;

        let r = folder_content
            .metadata
            .contents
            .and_then(|files| {
                files
                    .into_iter()
                    .filter_map(|m| match m {
                        Metadata::MetadataFile(m) => Some(m),
                        Metadata::MetadataFolder(_) => None,
                    })
                    .find(|f| f.common.name.as_ref().unwrap() == filename)
            })
            .map(|metadata| metadata.fileid);

        Ok(r)
    }
}
