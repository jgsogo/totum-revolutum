use async_trait::async_trait;

use pcloud_sdk::methods::streaming::getfilelink::{GetFileLink, GetFileLinkInput};
use pcloud_sdk::types::PCloudFile;

use crate::diff::File;

pub struct RemoteFile<HttpClient: impl pcloud_sdk::client::GetFileLink> {
    pcloud: pcloud_sdk::client::HttpClient,
    file: PCloudFile,
}

impl RemoteFile {
    pub fn new(file: PCloudFile, pcloud: pcloud_sdk::client::HttpClient) -> Self {
        Self { file, pcloud }
    }
}

#[async_trait]
impl File for RemoteFile {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        let input = GetFileLinkInput::new_from_file(self.file.clone());
        let r = self.pcloud.getfilelink(&input).await?;
        todo!()
    }
}
