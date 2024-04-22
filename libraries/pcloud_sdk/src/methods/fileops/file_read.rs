use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::fileops::FileDescriptor;
use crate::methods::params::Params;

pub const ENDPOINT: &str = "/file_read";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRead {
    pub bytes: Vec<u8>,
}

#[async_trait]
pub trait GetFileRead {
    async fn file_read(&self, descriptor: FileDescriptor, count: u64) -> Result<FileRead>;
}

#[async_trait]
impl<T: client::ClientBytes> GetFileRead for T {
    async fn file_read(&self, descriptor: FileDescriptor, count: u64) -> Result<FileRead> {
        let mut params = descriptor.into_params()?;
        params.insert("count".to_string(), count.to_string());

        let bytes = self.get_bytes(ENDPOINT, params).await?;
        Ok(FileRead { bytes })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_read() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get_bytes()
            .times(1)
            .returning(|endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/file_read");
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                assert_eq!(params.get("count"), Some(&"100".to_string()));
                let r = Vec::<u8>::new();
                Ok(r)
            });
        let _r = client.file_read(42, 100).await?;
        Ok(())
    }
}
