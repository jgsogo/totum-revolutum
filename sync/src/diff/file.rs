use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait File {
    async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>;

    async fn write_all(&mut self, buf: &[u8]) -> Result<()>;
}
