use anyhow::Result;
use http_utils::rest::RESTClient;
use http_utils::HttpClient;
use std::collections::HashMap;

pub trait RebrickableClient: RESTClient {}

#[derive(Debug)]
pub struct RebrickableClientImpl {
    api_key: String,
    http_client: reqwest::Client,
}

impl HttpClient for RebrickableClientImpl {
    fn build_url(&self, endpoint: &str) -> String {
        format!("https://rebrickable.com/api/v3{}", endpoint)
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    fn params(&self, mut params: HashMap<String, String>) -> HashMap<String, String> {
        params.insert("key".to_string(), self.api_key.clone());
        params
    }
}

impl RESTClient for RebrickableClientImpl {}

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
