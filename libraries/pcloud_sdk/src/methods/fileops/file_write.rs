use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use http_utils::rest::RESTClient;

use crate::methods::fileops::FileDescriptor;
use crate::methods::params::Params;
use crate::utils;

pub const ENDPOINT: &str = "/file_write";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileWrite {
    pub bytes: u64,
}

#[async_trait]
pub trait PostFileWrite {
    async fn file_write(&self, descriptor: FileDescriptor, data: &[u8]) -> Result<FileWrite>;
}

#[async_trait]
impl<T: RESTClient> PostFileWrite for T {
    async fn file_write(&self, descriptor: FileDescriptor, data: &[u8]) -> Result<FileWrite> {
        let data = utils::http::file_write(&mut data.to_owned(), "filename")?;
        let ret = RESTClient::post::<FileWrite>(self, ENDPOINT, HeaderMap::default(), descriptor.into_params()?, data)
            .await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_write() -> Result<()> {
        let mut client = MockLocalClient::new();
        let mut data = "gaudeamus igitur".as_bytes().to_vec();

        let bdata = utils::http::file_write(&mut data.clone(), "filename")?;
        client.expect_post().times(1).returning(
            move |endpoint, headers: HeaderMap, params: HashMap<_, _>, posted_data: Vec<u8>| {
                assert_eq!(endpoint, "/file_write");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                assert_eq!(posted_data, bdata);
                assert_eq!(headers.len(), 0);
                Ok(FileWrite { bytes: 10 })
            },
        );

        let r = client.file_write(42, &mut data).await?;
        assert_eq!(r.bytes, 10);
        Ok(())
    }
}
