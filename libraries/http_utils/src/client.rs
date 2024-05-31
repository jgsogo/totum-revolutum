use anyhow::Result;
use async_trait::async_trait;
use headers::HeaderMap;
use reqwest::Response;
use std::collections::HashMap;

#[async_trait]
/// A reusable HTTP client
pub trait HttpClient: Sync {
    /// Build the URL to call from the given endpoint and internal data (schema and hostname).
    fn build_url(&self, endpoint: &str) -> String;

    /// Returns the actual HTTP Client to use
    fn http_client(&self) -> &reqwest::Client;

    /// Populate extra headers. This method runs for all requests
    fn headers(&self, headers: HeaderMap) -> HeaderMap {
        headers
    }

    /// Populate extra query params. This method runs for all requests
    fn params(&self, params: HashMap<String, String>) -> HashMap<String, String> {
        params
    }

    /// Runs GET request to the given `endpoint` (URL will be built using [`self.build_url`])
    async fn get<T: Into<HashMap<String, String>> + Send + 'static>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        params: T,
    ) -> Result<Response> {
        let url = self.build_url(endpoint);
        let params = params.into();
        tracing::debug!("GET '{url}'");
        let request = self
            .http_client()
            .get(url)
            .headers(self.headers(headers))
            .query(&self.params(params));
        let response = request.send().await?;
        Ok(response)
    }

    /// Runs POST request to the given `endpoint` (URL will be built using [`self.build_url`])
    async fn post<T: Into<HashMap<String, String>> + Send + 'static>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        params: T,
        data: Vec<u8>,
    ) -> Result<Response> {
        let url = self.build_url(endpoint);
        let params = params.into();
        tracing::debug!("POST '{url}'");
        let request = self
            .http_client()
            .post(url)
            .headers(self.headers(headers))
            .query(&self.params(params))
            .body(reqwest::Body::from(data));
        let response = request.send().await?;
        Ok(response)
    }
}
