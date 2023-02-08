use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;

#[derive(Serialize, Deserialize, Debug)]
pub struct APIServer {
    binapi: Vec<String>,
    api: Vec<String>,
}

#[async_trait]
pub trait GetAPIServer: client::Client {
    async fn getapiserver(&self) -> Result<APIServer> {
        self.get::<APIServer>("/getapiserver", HashMap::new()).await
    }
}

impl<T: client::Client> GetAPIServer for T {}

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
            .returning(|endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/getapiserver");
                assert!(params.is_empty());
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
