use std::collections::HashMap;

use async_trait::async_trait;
use hyper::client::connect::Connect;
use serde::{Deserialize, Serialize};

use crate::client;

#[derive(Serialize, Deserialize, Debug)]
pub struct APIServer {
    binapi: Vec<String>,
    api: Vec<String>,
}

#[async_trait]
pub trait GetAPIServer<C>: client::Client<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    async fn getapiserver(&self) -> Result<APIServer, hyper::Error>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        let url = format!("https://{}/getapiserver", self.hostname());
        let apiserver = self.get::<APIServer>(&url, HashMap::new()).await?;
        Ok(apiserver)
    }
}

impl<C, T: client::Client<C>> GetAPIServer<C> for T where C: Connect + Clone + Send + Sync + 'static {}
