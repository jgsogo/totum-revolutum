use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path};
use itertools::any;

use crate::methods::folder::createfolderifnotexists::GetCreateFolderIfNotExists;
use crate::types::FolderID;
use crate::{Error, Result};

use super::GetFolderID;

#[async_trait]
/// Create the folder given by `path` inside folder given by `folder` (defaults to root)
pub trait GetCreateFolderIfNotExistsAll {
    async fn createfolderifnotexists_all(
        &self,
        folderid: &FolderID,
        path: impl AsRef<Utf8Path> + Send,
    ) -> Result<FolderID>;
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
            return Err(Error::InputDataEror(
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
}
