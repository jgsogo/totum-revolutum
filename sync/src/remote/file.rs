use async_trait::async_trait;
use futures::AsyncWriteExt;
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::FileOpen;
use pcloud_sdk::methods::fileops::file_read::GetFileRead;
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;

use crate::diff::File;

pub struct RemoteFile<HttpClient: GetFileRead + PostFileWrite + GetFileClose> {
    // TODO: This should be a reference &HttpClient, as every file can live as long as
    //  its filesystem will live... and the filesystem is one-to-one relationship with
    //  the httpclient used to connect to it.
    pcloud: HttpClient,
    file: FileOpen,
}

impl<HttpClient: GetFileRead + PostFileWrite + GetFileClose> RemoteFile<HttpClient> {
    pub fn new(file: FileOpen, pcloud: HttpClient) -> Self {
        Self { file, pcloud }
    }
}

#[async_trait]
impl<HttpClient: GetFileRead + PostFileWrite + GetFileClose + Sync + Send> File for RemoteFile<HttpClient> {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        // TODO: Read chunks until exhausted
        let content = self.pcloud.file_read(self.file.fd, 100).await?;
        buf.write_all(&*content.bytes).await?;
        Ok(content.bytes.len())
    }

    async fn write_all(&mut self, buf: &Vec<u8>) -> anyhow::Result<()> {
        let _r = self.pcloud.file_write(self.file.fd, buf).await?;
        Ok(())
    }
}
