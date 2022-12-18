use async_trait::async_trait;

use pcloud_sdk::methods::streaming::getfilelink::{GetFileLink, GetFileLinkInput};
use pcloud_sdk::types::PCloudFile;

use crate::diff::File;

pub struct RemoteFile<'fs, HttpClient: GetFileLink>
where
    HttpClient: 'fs,
{
    pcloud: &'fs HttpClient,
    file: PCloudFile,
}

impl<'fs, HttpClient: GetFileLink> RemoteFile<'fs, HttpClient> {
    pub fn new(file: PCloudFile, pcloud: &'fs HttpClient) -> Self {
        Self { file, pcloud }
    }
}

#[async_trait]
impl<'fs, HttpClient: GetFileLink + Sync + Send> File for RemoteFile<'fs, HttpClient> {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        let input = GetFileLinkInput::new_from_file(self.file.clone());
        let r = self.pcloud.getfilelink(&input).await?;
        todo!()
    }
}
