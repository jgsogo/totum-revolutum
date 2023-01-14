use std::io::Write;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use flume::Sender;
use tracing::warn;

use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::FileOpen;
use pcloud_sdk::methods::fileops::file_read::GetFileRead;
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;
use pcloud_sdk::methods::fileops::FileDescriptor;

use crate::filesystem::File;

pub const CHUNK_SIZE: usize = 512; // Just a guess of _optimal package size over a network_

pub struct RemoteFile<HttpClient: GetFileRead + PostFileWrite + GetFileClose + Sync + Send> {
    // TODO: This should be a reference &HttpClient, as every file can live as long as
    //  its filesystem will live... and the filesystem is one-to-one relationship with
    //  the httpclient used to connect to it.
    pcloud: Arc<HttpClient>,
    file: FileOpen,
    tx_file_close: Sender<FileDescriptor>,
}

impl<HttpClient: GetFileRead + PostFileWrite + GetFileClose + Sync + Send> RemoteFile<HttpClient> {
    pub fn new(file: FileOpen, pcloud: Arc<HttpClient>, tx_file_close: Sender<FileDescriptor>) -> Self {
        Self {
            file,
            pcloud,
            tx_file_close,
        }
    }
}

impl<HttpClient: GetFileRead + PostFileWrite + GetFileClose + Sync + Send> Drop for RemoteFile<HttpClient> {
    fn drop(&mut self) {
        // The filesystem takes care of closing the file
        if let Err(e) = self.tx_file_close.send(self.file.fd) {
            warn!("Error closing the file on drop action: {e}");
        }
    }
}

#[async_trait]
impl<HttpClient: GetFileRead + PostFileWrite + GetFileClose + Sync + Send> File for RemoteFile<HttpClient> {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        // Implementation is taken from async-std-1.12.0/src/io/read/read_to_end.rs, I'm not that smart
        struct Guard<'a> {
            buf: &'a mut Vec<u8>,
            len: usize,
        }

        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                unsafe {
                    self.buf.set_len(self.len);
                }
            }
        }

        let start_len = buf.len();
        let mut g = Guard { len: buf.len(), buf };
        let ret;
        loop {
            if g.len == g.buf.len() {
                unsafe {
                    g.buf.reserve(CHUNK_SIZE);
                    let capacity = g.buf.capacity();
                    g.buf.set_len(capacity);
                    let new_chunk = &mut g.buf[g.len..];
                    std::ptr::write_bytes(new_chunk.as_mut_ptr(), 0, new_chunk.len());
                }
            }

            match self.read(&mut g.buf[g.len..]).await {
                Ok(0) => {
                    ret = Ok(g.len - start_len);
                    break;
                }
                Ok(n) => {
                    g.len += n;
                }
                Err(e) => {
                    ret = Err(e);
                    break;
                }
            }
        }
        ret
    }

    async fn read(&mut self, mut buf: &mut [u8]) -> Result<usize> {
        let content = self.pcloud.file_read(self.file.fd, buf.len() as u64).await?;
        buf.write_all(&content.bytes)?;
        Ok(content.bytes.len())
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        let _r = self.pcloud.file_write(self.file.fd, buf).await?;
        Ok(())
    }

    async fn sync_all(&mut self) -> Result<()> {
        self.pcloud.file_close(self.file.fd).await
    }
}
