use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::methods::fileops::FileDescriptor;
use crate::{client, utils};

pub const ENDPOINT: &str = "/file_write";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileWrite {
    pub bytes: u64,
}

#[async_trait]
pub trait PostFileWrite: client::Client {
    async fn file_write(&self, descriptor: FileDescriptor, data: &Vec<u8>) -> Result<FileWrite> {
        let mut params = HashMap::new();
        params.insert("fd".to_string(), descriptor.to_string());

        let data = utils::http::file_write(&mut data.clone(), "filename")?;
        let ret = self.post::<FileWrite>(ENDPOINT, params, data).await?;
        Ok(ret)
    }
}

impl<T: client::Client> PostFileWrite for T {}

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_write() -> Result<()> {
        let mut client = MockLocalClient::new();
        let mut data = "gaudeamus igitur".as_bytes().to_vec();

        let bdata = utils::http::file_write(&mut data.clone(), "filename")?;
        client
            .expect_post()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>, posted_data: Vec<u8>| {
                assert_eq!(endpoint, "/file_write");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                assert_eq!(posted_data, bdata);
                Ok(FileWrite { bytes: 10 })
            });

        let r = client.file_write(42, &mut data).await?;
        assert_eq!(r.bytes, 10);
        Ok(())
    }
}
