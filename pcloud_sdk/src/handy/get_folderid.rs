use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8Path;

use crate::methods::folder::{listfolder, ListFolderInput};
use crate::types::{Folder, FolderID};

#[async_trait]
/// Returns the `FolderID` for the given path. It will fail if the folder doesn't exist.
pub trait GetFolderID {
    async fn get_folderid(&self, path: &Utf8Path) -> Result<FolderID>;
}

#[async_trait]
impl<T: listfolder::GetListFolder + Sync> GetFolderID for T {
    async fn get_folderid(&self, path: &Utf8Path) -> Result<FolderID> {
        let input = ListFolderInput::new(Folder::Path(path.to_path_buf()));
        let filtermeta = vec!["folderid"];
        let r = self.listfolder_with_filtermeta(input, filtermeta).await?;
        Ok(r.metadata.folderid)
    }
}
