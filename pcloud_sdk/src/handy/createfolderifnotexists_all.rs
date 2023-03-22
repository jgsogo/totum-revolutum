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
        let root_folder: Utf8PathBuf = "/".into();
        match folder {
            None => {
                if !path.is_absolute() {
                    bail!("If no parent folder is given, the `path` needs to be an absolute one")
                }
                if path.starts_with("/..") {
                    bail!("Folder '{path}' is outside the root folder")
                }
                let root_folderid = self.get_folderid(&root_folder).await?;
                Ok((root_folderid, path.strip_prefix(root_folder)?.into()))
            }
            Some(fid) => {
                if path.starts_with("/..") || path.starts_with("..") {
                    bail!("Folder '{path}' is outside the given parent")
                }
                Ok((fid, path.strip_prefix(root_folder).unwrap_or(&path).into()))
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

#[cfg(test)]
mod tests {
    use super::*;

    struct HelperTraitMock;

    #[async_trait]
    impl GetFolderID for HelperTraitMock {
        async fn get_folderid(&self, _path: &Utf8Path) -> Result<FolderID> {
            Ok(FolderID(42))
        }
    }

    #[tokio::test]
    async fn test_helper_trait_without_folderid() -> Result<()> {
        let trait_impl = HelperTraitMock {};

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(None, &Utf8PathBuf::from("/abs/path"))
                .await?;

            assert_eq!(folder, FolderID(42));
            assert_eq!(path, Utf8PathBuf::from("abs/path"));
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(None, &Utf8PathBuf::from("/abs/../path"))
                .await?;

            assert_eq!(folder, FolderID(42));
            assert_eq!(path, Utf8PathBuf::from("path"));
        }

        {
            let r = trait_impl
                .get_folder_and_path(None, &Utf8PathBuf::from("/../outside/root"))
                .await;

            assert!(r.is_err());
            assert_eq!(
                r.err().unwrap().to_string(),
                "Folder '/../outside/root' is outside the root folder".to_string()
            );
        }

        {
            let r = trait_impl
                .get_folder_and_path(None, &Utf8PathBuf::from("a/relative/path"))
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
                .get_folder_and_path(Some(FolderID(54)), &Utf8PathBuf::from("/abs/path"))
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path, Utf8PathBuf::from("abs/path"));
        }

        {
            let r = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &Utf8PathBuf::from("/../outside/root"))
                .await;

            assert!(r.is_err());
            assert_eq!(
                r.err().unwrap().to_string(),
                "Folder '/../outside/root' is outside the given parent".to_string()
            );
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &Utf8PathBuf::from("a/relative/path"))
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path, Utf8PathBuf::from("a/relative/path"));
        }

        {
            let (folder, path) = trait_impl
                .get_folder_and_path(Some(FolderID(54)), &Utf8PathBuf::from("a/relative/../path"))
                .await?;

            assert_eq!(folder, FolderID(54));
            assert_eq!(path, Utf8PathBuf::from("a/path"));
        }
        Ok(())
    }
}
