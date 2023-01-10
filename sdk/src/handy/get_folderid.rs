use std::path::Path;

use anyhow::{anyhow, Result};
use async_trait::async_trait;

use crate::methods::folder::listfolder;
use crate::types::FolderID;

#[async_trait]
pub trait GetFolderID: listfolder::GetListFolder {
    async fn get_folderid(&self, path: &Path) -> Result<FolderID> {
        let listfolder_input = listfolder::ListFolderInput::new_from_path(Some(path.to_str().unwrap().to_string()));
        let filtermeta = vec!["folderid"];
        let r = self.listfolder_with_filtermeta(&listfolder_input, filtermeta).await?;
        r.metadata
            .folderid
            .ok_or_else(|| anyhow!("Cannot get folderID for given path"))
    }
}

impl<T: listfolder::GetListFolder> GetFolderID for T {}
