use crate::utils::http;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[async_trait]
pub trait Client: Sync {
    async fn get<T>(&self, endpoint: &str, params: HashMap<String, String>) -> anyhow::Result<T>
    where
        T: DeserializeOwned + 'static;

    async fn post<T>(&self, endpoint: &str, params: HashMap<String, String>, data: Vec<u8>) -> anyhow::Result<T>
    where
        T: DeserializeOwned + 'static;
}

#[derive(Debug)]
pub struct HttpClient {
    api_key: String,
    http_client: reqwest::Client,
    secure: bool,
}

impl HttpClient {
    pub fn new(api_key: String, secure: bool) -> HttpClient {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        HttpClient {
            api_key,
            http_client: client,
            secure,
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        let schema = if self.secure { "https" } else { "http" };
        format!("{}://rebrickable.com{}", schema, endpoint)
    }
}

#[async_trait]
impl Client for HttpClient {
    async fn get<T>(&self, endpoint: &str, mut params: HashMap<String, String>) -> anyhow::Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        params.insert("key".to_string(), self.api_key.clone());
        http::get::<T>(self.http_client.clone(), &url, params).await
    }

    async fn post<T>(&self, endpoint: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> anyhow::Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        params.insert("key".to_string(), self.api_key.clone());
        http::post::<T>(self.http_client.clone(), &url, params, data).await
    }
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        HttpClient {
            api_key: self.api_key.clone(),
            http_client: self.http_client.clone(),
            secure: self.secure,
        }
    }
}
