use crate::{Error, Result};
use http_utils::rest::RESTClient;
use http_utils::HttpClient;
use std::collections::HashMap;

pub trait RebrickableClient: RESTClient<RESTClientError = Error> {}

#[derive(Debug)]
pub struct RebrickableClientImpl {
    api_key: String,
    http_client: reqwest::Client,
}

impl HttpClient for RebrickableClientImpl {
    type Error = Error;

    fn build_url(&self, endpoint: &str) -> String {
        format!("https://rebrickable.com/api/v3{}", endpoint)
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert("key".to_string(), self.api_key.clone());
        params
    }
}

impl RESTClient for RebrickableClientImpl {
    type RESTClientError = Error;
}

impl RebrickableClient for RebrickableClientImpl {}

impl RebrickableClientImpl {
    pub fn new(api_key: String) -> Result<Self> {
        let client = reqwest::ClientBuilder::new().build()?;
        Ok(Self {
            api_key,
            http_client: client,
        })
    }
}
