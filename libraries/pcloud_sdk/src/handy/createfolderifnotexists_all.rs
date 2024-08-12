use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path};
use itertools::any;

use crate::methods::folder::createfolderifnotexists::GetCreateFolderIfNotExists;
use crate::types::{FolderID, RemotePath};
use crate::{Error, Result};

use super::GetFolderID;

#[async_trait]
pub trait GetCreateFolderIfNotExistsAll {
    /// Create the folder given by `path` inside folder given by `folderid`
    async fn createfolderifnotexists_all(
        &self,
        folderid: &FolderID,
        path: impl AsRef<Utf8Path> + Send,
    ) -> Result<FolderID>;

    /// Create the folder given by `path` starting from the root folder
    async fn createfolderifnotexists_all_from_root(&self, path: &RemotePath) -> Result<FolderID>;
}

#[async_trait]
impl<Client: GetFolderID + GetCreateFolderIfNotExists + Sync> GetCreateFolderIfNotExistsAll for Client {
    async fn createfolderifnotexists_all(
        &self,
        folderid: &FolderID,
        path: impl AsRef<Utf8Path> + Send,
    ) -> Result<FolderID> {
        let mut folderid = folderid.clone();

        // Check path components
        if any(path.as_ref().components(), |c| {
            !matches!(c, Utf8Component::Normal { .. })
        }) {
            return Err(Error::InputDataError(
                "Input argument `path` invalid: only regular path components are allowed".to_string(),
            ));
        }

        // Create all the directories
        for cmp in path.as_ref().components() {
            let Utf8Component::Normal(cmp) = cmp else {
                panic!("This has been checked above")
            };
            let r = self.createfolderifnotexists(&folderid, cmp).await?;
            folderid = r.metadata.folderid;
        }

        Ok(folderid)
    }

    async fn createfolderifnotexists_all_from_root(&self, path: &RemotePath) -> Result<FolderID> {
        let root = RemotePath::root();
        let root_folderid = self.get_folderid(&root).await?;
        self.createfolderifnotexists_all(&root_folderid, path.relative_to(&root)?)
            .await
    }
}
