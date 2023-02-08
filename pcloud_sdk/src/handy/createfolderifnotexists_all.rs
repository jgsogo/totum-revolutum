use std::path::{Component, Path};

use anyhow::{bail, Result};
use async_trait::async_trait;
use tracing::debug;

use crate::methods::folder::createfolderifnotexists;
use crate::types::FolderID;
use crate::utils::normalize_path;

use super::GetFolderID;

#[async_trait]
pub trait GetCreateFolderIfNotExistsAll: GetFolderID + createfolderifnotexists::GetCreateFolderIfNotExists {
    async fn createfolderifnotexists_all(&self, path: &Path) -> Result<FolderID> {
        let path = normalize_path(path);
        debug!("Create all folders (if not exist): '{}'", path.display());
        assert!(path.is_absolute());

        let mut folderid = self.get_folderid(Path::new("/")).await?;
        for cmp in path.components() {
            match cmp {
                Component::RootDir => continue,
                Component::Normal(p) => {
                    let input = createfolderifnotexists::CreateFolderIfNotExistsInput::FolderAndName(
                        folderid.clone(),
                        p.to_string_lossy().to_string(),
                    );
                    let r = self.createfolderifnotexists(&input).await?;
                    folderid = r.metadata.folderid.unwrap();
                }
                _ => bail!("Component in path not expected"),
            }
        }
        Ok(folderid)
    }
}

impl<T: GetFolderID + createfolderifnotexists::GetCreateFolderIfNotExists> GetCreateFolderIfNotExistsAll for T {}
