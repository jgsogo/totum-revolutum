use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;

use http_utils::rest::RESTClient;

use crate::methods::fileops::FileDescriptor;
use crate::methods::params::Params;

pub const ENDPOINT: &str = "/file_close";

#[async_trait]
pub trait GetFileClose {
    async fn file_close(&self, descriptor: FileDescriptor) -> Result<()>;
}

#[async_trait]
impl<T: RESTClient> GetFileClose for T {
    async fn file_close(&self, descriptor: FileDescriptor) -> Result<()> {
        RESTClient::get::<()>(self, ENDPOINT, HeaderMap::default(), descriptor.into_params()?).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_close() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/file_close");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(())
            });
        let _r = client.file_close(42).await?;
        Ok(())
    }
}
