use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Result;
use async_trait::async_trait;
use reqwest;
use serde::de::DeserializeOwned;

use crate::methods::oauth2;
use crate::utils::http;

use super::data;

#[async_trait]
pub trait Client {
    async fn get<T>(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned + 'static;

    async fn post<T>(&self, endpoint: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> Result<T>
    where
        T: DeserializeOwned + 'static;
}

#[derive(Debug)]
pub struct HttpClient {
    pub oauth2_token: data::oauth2token::OAuth2Token,
    http_client: reqwest::Client,
}

impl HttpClient {
    pub fn new(oauth2_token: data::oauth2token::OAuth2Token) -> HttpClient {
        HttpClient {
            oauth2_token,
            http_client: reqwest::Client::new(),
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        format!("https://{}{}", self.oauth2_token.hostname(), endpoint)
    }

    pub async fn authorize(app: data::app_client_data::AppClientData, address: SocketAddr) -> Result<HttpClient> {
        let client = reqwest::Client::new();
        let oauth2 = oauth2::authorize_oauth2(client.clone(), app, address).await?;
        Ok(HttpClient {
            oauth2_token: oauth2,
            http_client: client,
        })
    }

    async fn get<T>(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token.clone();
        params.insert("access_token".to_string(), access_token);
        http::get::<T>(self.http_client.clone(), &url, params).await
    }

    async fn post<T>(&self, endpoint: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token.clone();
        params.insert("access_token".to_string(), access_token);
        http::post::<T>(self.http_client.clone(), &url, params, data).await
    }
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        HttpClient {
            oauth2_token: self.oauth2_token.clone(),
            http_client: self.http_client.clone(),
        }
    }
}
