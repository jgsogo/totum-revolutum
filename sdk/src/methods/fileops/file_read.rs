use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::fileops::FileDescriptor;

pub const ENDPOINT: &str = "/file_read";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileRead {
    pub bytes: Vec<u8>,
}

#[async_trait]
pub trait GetFileRead: client::Client {
    async fn file_read(&self, descriptor: FileDescriptor, count: u64) -> Result<FileRead> {
        let mut params = HashMap::new();
        params.insert("fd".to_string(), descriptor.to_string());
        params.insert("count".to_string(), count.to_string());

        let bytes = self.get_bytes(ENDPOINT, params).await?;
        Ok(FileRead { bytes })
    }
}

impl<T: client::Client> GetFileRead for T {}

#[cfg(test)]
mod tests {
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
