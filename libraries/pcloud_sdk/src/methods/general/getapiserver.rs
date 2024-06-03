use std::collections::HashMap;

use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use http_utils::rest::RESTClient;

use crate::client::PCloudClient;
use crate::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct APIServer {
    binapi: Vec<String>,
    api: Vec<String>,
}

#[async_trait]
pub trait GetAPIServer {
    async fn getapiserver(&self) -> Result<APIServer>;
}

#[async_trait]
impl<T: PCloudClient> GetAPIServer for T {
    async fn getapiserver(&self) -> Result<APIServer> {
        RESTClient::get(self, "/getapiserver", HeaderMap::default(), &HashMap::new()).await
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_getapiserver() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, "/getapiserver");
                assert!(params.is_empty());
                assert_eq!(headers.len(), 0);
                Ok(APIServer {
                    binapi: vec!["binapi".into()],
                    api: vec!["api".into()],
                })
            });

        let apiserver = client.getapiserver().await?;
        assert_eq!(apiserver.binapi, vec!["binapi".to_string()]);
        assert_eq!(apiserver.api, vec!["api".to_string()]);
        Ok(())
    }
}
