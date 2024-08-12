use async_std::fs::File as AsyncFile;
use async_trait::async_trait;

use crate::File;
use crate::{Error, Result};

#[async_trait]
impl File for AsyncFile {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        futures::io::AsyncReadExt::read_to_end(self, buf)
            .await
            .map_err(Error::IoError)
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        futures::io::AsyncReadExt::read(self, buf).await.map_err(Error::IoError)
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        futures::io::AsyncWriteExt::write_all(self, buf)
            .await
            .map_err(Error::IoError)
    }

    async fn sync_all(&self) -> Result<()> {
        AsyncFile::sync_all(self).await.map_err(Error::IoError)
    }
}

#[async_trait]
impl File for std::fs::File {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
        std::io::Read::read_to_end(self, buf).map_err(Error::IoError)
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        std::io::Read::read(self, buf).map_err(Error::IoError)
    }

    async fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        std::io::Write::write_all(self, buf).map_err(Error::IoError)
    }

    async fn sync_all(&self) -> Result<()> {
        std::fs::File::sync_all(self).map_err(Error::IoError)
    }
}
