use anyhow::anyhow;
use async_trait::async_trait;
use futures::io::AsyncReadExt;

use crate::diff::File;

pub struct LocalFile {
    file: async_std::fs::File,
}

impl LocalFile {
    pub fn new(file: async_std::fs::File) -> Self {
        Self { file }
    }
}

#[async_trait]
impl File for LocalFile {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> anyhow::Result<usize> {
        self.file.read_to_end(buf).await.map_err(|e| anyhow!(e))
    }
}
