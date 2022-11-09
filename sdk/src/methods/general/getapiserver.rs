use std::collections::HashMap;

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
    async fn getapiserver(&self) -> Result<APIServer, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("https://{}/getapiserver", self.hostname());
        let apiserver = self.get::<APIServer>(&url, HashMap::new()).await?;
        Ok(apiserver)
    }
}

impl<T: client::Client> GetAPIServer for T {}
