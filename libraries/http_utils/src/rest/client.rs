use crate::HttpClient;
use anyhow::Result;
use async_trait::async_trait;
use headers::HeaderMap;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[async_trait]
pub trait RESTClient: HttpClient {
    /// Deserialize API call result to return type
    fn parse_response<T>(result: String) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let r = serde_json::from_str::<T>(&result)?;
        Ok(r)
    }

    /// Runs GET request to the given `endpoint` (URL will be built using [`self.build_url`]) with
    /// some `params`
    async fn get<T>(&self, endpoint: &str, headers: HeaderMap, params: HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let response = HttpClient::get(self, endpoint, headers, params).await?;
        Self::parse_response(response.text().await?)
    }

    async fn post<T>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        params: HashMap<String, String>,
        data: Vec<u8>,
    ) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let response = HttpClient::post(self, endpoint, headers, params, data).await?;
        Self::parse_response(response.text().await?)
    }
}
