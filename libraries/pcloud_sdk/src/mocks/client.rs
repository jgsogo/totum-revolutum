use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use headers::HeaderMap;
use mockall::mock;
use serde::de::DeserializeOwned;

use http_utils::rest::RESTClient;
use http_utils::HttpClient;

use crate::client::PCloudClient;

mock! {
    #[allow(dead_code)]
    pub LocalClient {}

    #[allow(dead_code)]
    impl Clone for LocalClient {
        fn clone(&self) -> Self;
    }

    #[allow(dead_code)]
    #[async_trait]
    impl HttpClient for LocalClient {
        fn build_url(&self, endpoint: &str) -> String;

        fn http_client(&self) -> &reqwest::Client;

        fn headers(&self, headers: HeaderMap) -> HeaderMap;

        fn params(&self, params: HashMap<String, String>) -> HashMap<String, String>;


    }

    #[allow(dead_code)]
    #[async_trait]
    impl RESTClient for LocalClient {
        fn parse_response<T>(result: String) -> Result<T>
        where
            T: DeserializeOwned + 'static;

        async fn get<T, TParams: Into<HashMap<String, String>> + Send+ 'static>(&self, endpoint: &str, headers: HeaderMap, params: TParams) -> Result<T>
        where
            T: DeserializeOwned + 'static;

        async fn post<T, TParams: Into<HashMap<String, String>> + Send+ 'static>(
            &self,
            endpoint: &str,
            headers: HeaderMap,
            params: TParams,
            data: Vec<u8>,
        ) -> Result<T>
        where
            T: DeserializeOwned + 'static;
    }

    #[allow(dead_code)]
    #[async_trait]
    impl PCloudClient for LocalClient {
        async fn get_bytes<TParams: Into<HashMap<String, String>> + Send+ 'static>(&self, endpoint: &str, mut params: TParams) -> Result<Vec<u8>>;
    }
}
