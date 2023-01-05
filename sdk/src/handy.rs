use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::{Component, Path};

use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use tracing::debug;

use crate::methods::folder::createfolderifnotexists::GetCreateFolderIfNotExists;
use crate::methods::folder::{createfolderifnotexists, listfolder, ListFolderInput};
use crate::methods::streaming::getfilelink;
use crate::types::FolderID;
use crate::utils::normalize_path;

use super::progress_bar;

#[async_trait]
pub trait GetFolderID: listfolder::GetListFolder {
    async fn get_folderid(&self, path: &Path) -> Result<FolderID> {
        let listfolder_input = ListFolderInput::new_from_path(Some(path.to_str().unwrap().to_string()));
        let filtermeta = vec!["folderid"];
        let r = self.listfolder_with_filtermeta(&listfolder_input, filtermeta).await?;
        r.metadata
            .folderid
            .ok_or_else(|| anyhow!("Cannot get folderID for given path"))
    }
}

#[async_trait]
pub trait GetCreateFolderIfNotExistsAll: GetFolderID + GetCreateFolderIfNotExists {
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

#[async_trait]
pub trait GetFileLinkAndDownload: getfilelink::GetFileLink {
    async fn getfilelink_and_download(
        &self,
        file_link: &getfilelink::GetFileLinkInput,
        path: &Path,
        pb_builder: &dyn progress_bar::ProgressBarBuilder,
    ) -> Result<()> {
        let r = self.getfilelink(file_link).await?;
        let url = &format!("https://{}{}", r.hosts.first().unwrap(), r.path);
        debug!("Download file from '{}'", &url);

        // Reqwest setup
        // TODO: it should use underlying reqwest::Client
        let res = reqwest::ClientBuilder::default()
            .build()?
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to GET from '{url}': {e}"))?;
        let total_size = res
            .content_length()
            .ok_or_else(|| anyhow!("Failed to get content length from '{}'", &url))?;

        // Progress bar setup
        let pb = pb_builder.build(total_size);
        let (_, url_filename) = url.rsplit_once('/').unwrap();
        pb.set_message(&format!("Downloading '{}'", url_filename));

        // download chunks
        let mut file = File::create(path).map_err(|e| anyhow!("Failed to create file '{}': {e}", path.display()))?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item.map_err(|e| anyhow!("Error while downloading file: {e}"))?;
            file.write_all(&chunk)
                .map_err(|e| anyhow!("Error while writing to file: {e}"))?;
            downloaded = min(downloaded + (chunk.len() as u64), total_size);
            pb.set_position(downloaded);
        }

        pb.finish();
        return Ok(());
    }
}

impl<T: listfolder::GetListFolder> GetFolderID for T {}
impl<T: GetFolderID + GetCreateFolderIfNotExists> GetCreateFolderIfNotExistsAll for T {}
impl<T: getfilelink::GetFileLink> GetFileLinkAndDownload for T {}
