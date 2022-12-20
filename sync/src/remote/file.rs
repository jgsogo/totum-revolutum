use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::anyhow;
use async_std::fs::File as AsyncFile;
use async_trait::async_trait;
use futures::io::AsyncReadExt;

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
}

// Implementing this trait is required to implement `async_std::io::Write`. See: https://docs.rs/async-std/0.99.4/async_std/io/trait.Write.html
// impl<HttpClient: GetFileLink + Sync + Send> futures::io::AsyncWrite for RemoteFile<HttpClient> {
//     fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
//         unsafe {
//             let a = Pin::get_unchecked_mut(self);
//             let boxed = Pin::new(&mut a.file);
//             boxed.poll_write(cx, buf)
//         }
//     }
//
//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
//         unsafe {
//             let a = Pin::get_unchecked_mut(self);
//             let boxed = Pin::new(&mut a.file);
//             boxed.poll_flush(cx)
//         }
//     }
//
//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
//         unsafe {
//             let a = Pin::get_unchecked_mut(self);
//             let boxed = Pin::new(&mut a.file);
//             boxed.poll_close(cx)
//         }
//     }
// }
