use async_trait::async_trait;

use pcloud_sdk::methods::streaming::getfilelink::{GetFileLink, GetFileLinkInput};
use pcloud_sdk::types::PCloudFile;

use crate::diff::File;

pub struct RemoteFile<HttpClient: GetFileLink> {
    // TODO: This should be a reference &HttpClient, as every file can live as long as
    //  its filesystem will live... and the filesystem is one-to-one relationship with
    //  the httpclient used to connect to it.
    pcloud: HttpClient,
    file: PCloudFile,
}

impl<HttpClient: GetFileLink> RemoteFile<HttpClient> {
    pub fn new(file: PCloudFile, pcloud: HttpClient) -> Self {
        Self { file, pcloud }
    }
}

#[async_trait]
impl<HttpClient: GetFileLink + Sync + Send> File for RemoteFile<HttpClient> {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        let input = GetFileLinkInput::new_from_file(self.file.clone());
        let r = self.pcloud.getfilelink(&input).await?;
        todo!()
    }

    async fn write_all(&mut self, buf: &Vec<u8>) -> anyhow::Result<()> {
        todo!()
    }
}
