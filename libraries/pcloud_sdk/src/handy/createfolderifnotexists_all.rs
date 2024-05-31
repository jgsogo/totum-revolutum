use anyhow::{bail, Result};
use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path};
use itertools::any;

use crate::methods::folder::createfolderifnotexists;
use crate::types::{FolderID, RemotePath};

use super::GetFolderID;

pub enum BaseFolder<'a> {
    FolderID(&'a FolderID),
    RemotePath(&'a RemotePath),
}

impl<'a> From<&'a FolderID> for BaseFolder<'a> {
    fn from(value: &'a FolderID) -> Self {
        Self::FolderID(value)
    }
}

impl<'a> From<&'a RemotePath> for BaseFolder<'a> {
    fn from(value: &'a RemotePath) -> Self {
        Self::RemotePath(value)
    }
}

#[async_trait]
/// Create the folder given by `path` inside folder given by `folder` (defaults to root)
pub trait GetCreateFolderIfNotExistsAll {
    async fn createfolderifnotexists_all<'a, T: Into<BaseFolder<'a>> + Send>(
        &self,
        folder: T,
        path: impl AsRef<Utf8Path> + Send,
    ) -> Result<FolderID>;
}

#[async_trait]
impl<Client: GetFolderID + createfolderifnotexists::GetCreateFolderIfNotExists + Sync> GetCreateFolderIfNotExistsAll
    for Client
{
    async fn createfolderifnotexists_all<'a, T: Into<BaseFolder<'a>> + Send>(
        &self,
        folder: T,
        path: impl AsRef<Utf8Path> + Send,
    ) -> Result<FolderID> {
        let mut folderid = match folder.into() {
            BaseFolder::FolderID(fid) => fid.clone(),
            BaseFolder::RemotePath(remote_path) => self.get_folderid(remote_path).await?,
        };

        // Check path components
        if any(path.as_ref().components(), |c| {
            matches!(c, Utf8Component::Normal { .. })
        }) {
            bail!("Input argument `path` invalid: only regular path components are allowed")
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
