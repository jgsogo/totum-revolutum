use async_trait::async_trait;

use crate::methods::file::stat::GetStat;
use crate::types::{File, FileID, RemotePath};
use crate::Result;

#[async_trait]
pub trait GetFileID {
    /// Returns the [`FileID`] for the given path. It will fail if the file doesn't exist.
    async fn get_fileid(&self, path: &RemotePath) -> Result<FileID>;
}

#[async_trait]
impl<T: GetStat + Sync> GetFileID for T {
    async fn get_fileid(&self, path: &RemotePath) -> Result<FileID> {
        let file = File::RemotePath(path.to_owned());
        let stats = self.stat(&file).await?;
        Ok(stats.metadata.fileid)
    }
}
