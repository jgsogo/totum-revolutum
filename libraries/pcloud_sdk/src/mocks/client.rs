use std::collections::HashMap;

use crate::{Error, Result};
use async_trait::async_trait;
use headers::HeaderMap;
use mockall::mock;
use serde::de::DeserializeOwned;

use utils::http::rest::RESTClient;
use utils::http::{AddToParams, HttpClient};

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
        type Error = Error;

        fn build_url(&self, endpoint: &str) -> String;

        fn http_client(&self) -> &reqwest::Client;

        fn headers(&self, headers: HeaderMap) -> HeaderMap;

        fn params(&self) -> HashMap<String, String>;
    }

    #[allow(dead_code)]
    #[async_trait]
    impl RESTClient for LocalClient {
        fn parse_response<T>(result: String) -> std::result::Result<T, <MockLocalClient as HttpClient>::Error>
        where
            T: DeserializeOwned + 'static;

        async fn get<T, TParams: AddToParams + Sync + 'static>(&self, endpoint: &str, headers: HeaderMap, params: &TParams) -> std::result::Result<T, <MockLocalClient as HttpClient>::Error>
        where
            T: DeserializeOwned + 'static;

        async fn post<T, TParams: AddToParams + Sync + 'static>(
            &self,
            endpoint: &str,
            headers: HeaderMap,
            params: &TParams,
            data: Vec<u8>,
        ) -> std::result::Result<T, <MockLocalClient as HttpClient>::Error>
        where
            T: DeserializeOwned + 'static;
    }

    #[allow(dead_code)]
    #[async_trait]
    impl PCloudClient for LocalClient {
        async fn get_bytes<TParams: AddToParams + Sync + 'static>(&self, endpoint: &str, mut params: &TParams) -> Result<Vec<u8>>;
    }
}
