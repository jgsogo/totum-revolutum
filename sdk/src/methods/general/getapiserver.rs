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
        let url = format!("https://{}/getapiserver", self.hostname());
        let apiserver = self.get::<APIServer>(&url, HashMap::new()).await?;
        Ok(apiserver)
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
        client.expect_hostname().times(1).returning(|| "<hostname>".into());
        client
            .expect_access_token()
            .times(1)
            .returning(|| "<access_token>".into());
        client.expect_http_client().times(1).returning(|| 23);

        let apiserver = client.getapiserver().await?;
        Ok(())
    }
}
