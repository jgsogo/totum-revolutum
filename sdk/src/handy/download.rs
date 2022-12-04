use std::cmp::min;
use std::fs::File;
use std::io::Write;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use tracing::debug;

use crate::methods::streaming::getfilelink;

use crate::progress_bar;

/// Downloads a file to the given ˋpathˋ
#[async_trait]
pub trait GetFileLinkAndDonwload: getfilelink::GetFileLink {
    async fn getfilelink_and_download(
        &self,
        file_link: &getfilelink::GetFileLinkInput,
        path: &std::path::Path,
        pb_builder: &dyn progress_bar::ProgressBarBuilder,
    ) -> Result<()> {
        let r = self.getfilelink(file_link).await?;
        let url = &format!("https://{}{}", r.hosts.first().unwrap(), r.path);
        debug!("Download file from '{}'", &url);

        // Reqwest setup
        let res = self
            .http_client()
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

impl<T: getfilelink::GetFileLink> GetFileLinkAndDonwload for T {}
