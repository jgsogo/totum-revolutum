use std::str::FromStr;

use anyhow::{bail, Result};
use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
use tracing::debug;

use crate::methods::folder::createfolderifnotexists;
use crate::methods::folder::createfolderifnotexists::TargetFolder;
use crate::types::{FolderID, RemotePath};
use crate::utils::normalize_path;

use super::GetFolderID;

#[async_trait]
/// Create the folder given by `path` inside folder given by `folder` (defaults to root)
pub trait GetCreateFolderIfNotExistsAll {
    async fn createfolderifnotexists_all(&self, folder: Option<FolderID>, path: &RemotePath) -> Result<FolderID>;
}

#[async_trait]
trait _HelperTrait: GetFolderID {
    async fn get_folder_and_path(&self, folder: Option<FolderID>, path: &RemotePath) -> Result<(FolderID, RemotePath)> {
        let root_folder: Utf8PathBuf = "/".into();
        match folder {
            None => {
                let remote_root = RemotePath::from_str("path:/")?;
                let root_folderid = self.get_folderid(&remote_root).await?;
                Ok((root_folderid, remote_root.join(path)))
            }
            Some(fid) => Ok((fid, path.clone())),
        }
    }
}

impl<T: GetFolderID> _HelperTrait for T {}

#[async_trait]
impl<T: GetFolderID + createfolderifnotexists::GetCreateFolderIfNotExists + Sync> GetCreateFolderIfNotExistsAll for T {
    async fn createfolderifnotexists_all(&self, folder: Option<FolderID>, path: &RemotePath) -> Result<FolderID> {
        debug!("Create folder '{path}' (if not exists) (inside '{folder:?}')");
        let (mut folderid, path) = self.get_folder_and_path(folder, path).await?;
        for cmp in path.components() {
            if let Utf8Component::Normal(p) = cmp {
                let input = TargetFolder::FolderAndName((folderid.clone(), p.to_string()));
                let r = self.createfolderifnotexists(input).await?;
                folderid = r.metadata.folderid;
            }
        }
        Ok(folderid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct HelperTraitMock;

    #[async_trait]
    impl GetFolderID for HelperTraitMock {
        async fn get_folderid(&self, _path: &RemotePath) -> Result<FolderID> {
            Ok(FolderID(42))
        }
    }

    #[tokio::test]
    async fn test_helper_trait_without_folderid() -> Result<()> {
        let trait_impl = HelperTraitMock {};

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(None, &RemotePath::from_str("path:/abs/path")?)
                .await?;

            assert_eq!(folder, FolderID(42));
            assert_eq!(path.to_string(), "/abs/path");
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(None, &RemotePath::from_str("/abs/../path")?)
                .await?;

            assert_eq!(folder, FolderID(42));
            assert_eq!(path.to_string(), "/path");
        }

        {
            let r = trait_impl
                .get_folder_and_path(None, &RemotePath::from_str("/../outside/root")?)
                .await;

            assert!(r.is_err());
            assert_eq!(
                r.err().unwrap().to_string(),
                "Folder '/../outside/root' is outside the root folder".to_string()
            );
        }

        {
            let r = trait_impl
                .get_folder_and_path(None, &RemotePath::from_str("a/relative/path")?)
                .await;

            assert!(r.is_err());
            assert_eq!(
                r.err().unwrap().to_string(),
                "If no parent folder is given, the `path` needs to be an absolute one".to_string()
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_helper_trait_with_folderid() -> Result<()> {
        let trait_impl = HelperTraitMock {};

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &RemotePath::from_str("/abs/path")?)
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path.to_string(), "/abs/path");
        }

        {
            let r = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &RemotePath::from_str("/../outside/root")?)
                .await;

            assert!(r.is_err());
            assert_eq!(
                r.err().unwrap().to_string(),
                "Folder '/../outside/root' is outside the given parent".to_string()
            );
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &RemotePath::from_str("a/relative/path")?)
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path.to_string(), "a/relative/path");
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &RemotePath::from_str("a/relative/../path")?)
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path.to_string(), "a/path");
        }
        Ok(())
    }
}
