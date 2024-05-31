use crate::Result;
use async_trait::async_trait;
use http::HeaderMap;

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::fileops::FileDescriptor;

pub const ENDPOINT: &str = "/file_close";

#[async_trait]
pub trait GetFileClose {
    async fn file_close(&self, descriptor: FileDescriptor) -> Result<()>;
}

#[async_trait]
impl<T: PCloudClient> GetFileClose for T {
    async fn file_close(&self, descriptor: FileDescriptor) -> Result<()> {
        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &descriptor).await
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_close() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, query: &FileDescriptor| {
                assert_eq!(endpoint, "/file_close");
                assert_eq!(headers.len(), 0);
                assert_eq!(query, &FileDescriptor::new(42));
                Ok(())
            });
        let _r = client.file_close(FileDescriptor::new(42)).await?;
        Ok(())
    }
}
