use anyhow::{bail, Result};
use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use tracing::debug;

use crate::methods::folder::createfolderifnotexists;
use crate::methods::folder::createfolderifnotexists::TargetFolder;
use crate::types::FolderID;
use crate::utils::normalize_path;

use super::GetFolderID;

#[async_trait]
pub trait GetCreateFolderIfNotExistsAll {
    async fn createfolderifnotexists_all(&self, folder: Option<FolderID>, path: &Utf8Path) -> Result<FolderID>;
}

#[async_trait]
trait _HelperTrait: GetFolderID {
    async fn get_folder_and_path(&self, folder: Option<FolderID>, path: &Utf8Path) -> Result<(FolderID, Utf8PathBuf)> {
        let path = normalize_path(path);
        match folder {
            None => {
                if !path.is_absolute() {
                    bail!("If no parent folder is given, the `path` needs to be an absolute one")
                }
                let root_folder: Utf8PathBuf = "/".into();
                let root_folderid = self.get_folderid(&root_folder).await?;
                Ok((root_folderid, path.strip_prefix(root_folder)?.into()))
            }
            Some(fid) => {
                if path.starts_with("/..") || path.starts_with("..") {
                    bail!("Folder '{path}' is outside the given parent")
                }
                Ok((fid, path))
            }
        }
    }
}

impl<T: GetFolderID> _HelperTrait for T {}

#[async_trait]
impl<T: GetFolderID + createfolderifnotexists::GetCreateFolderIfNotExists + Sync> GetCreateFolderIfNotExistsAll for T {
    async fn createfolderifnotexists_all(&self, folder: Option<FolderID>, path: &Utf8Path) -> Result<FolderID> {
        debug!("Create folder '{path}' (if not exists) (inside '{folder:?}')");
        let (mut folderid, path) = self.get_folder_and_path(folder, path).await?;
        for cmp in path.components() {
            if let Utf8Component::Normal(p) = cmp {
                let input = TargetFolder::FolderAndName((folderid.clone(), p.to_string()));
                let r = self.createfolderifnotexists(input).await?;
                folderid = r.metadata.folderid.unwrap();
            }
        }
        Ok(folderid)
    }
}
