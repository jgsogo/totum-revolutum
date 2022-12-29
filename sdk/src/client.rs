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

    async fn get_bytes(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct HttpClient<Token: data::oauth2token::OAuth2Token> {
    pub oauth2_token: Token,
    http_client: reqwest::Client,
    secure: bool,
}

impl<Token: data::oauth2token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> HttpClient<Token> {
    pub fn new(oauth2_token: Token, secure: bool) -> HttpClient<Token> {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        HttpClient {
            oauth2_token,
            http_client: client,
            secure,
        }
    }

    fn build_url(&self, endpoint: &str) -> String {
        let schema = if self.secure { "https" } else { "http" };
        format!("{}://{}{}", schema, self.oauth2_token.hostname(), endpoint)
    }

    pub async fn authorize(
        app: data::app_client_data::AppClientData,
        address: SocketAddr,
    ) -> Result<HttpClient<Token>> {
        let client = reqwest::Client::new();
        let oauth2 = oauth2::authorize_oauth2(client.clone(), app, address).await?;
        Ok(HttpClient {
            oauth2_token: oauth2,
            http_client: client,
            secure: true,
        })
    }
}

#[async_trait]
impl<Token: data::oauth2token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> Client for HttpClient<Token> {
    async fn get<T>(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        http::get::<T>(self.http_client.clone(), &url, params).await
    }

    async fn post<T>(&self, endpoint: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        http::post::<T>(self.http_client.clone(), &url, params, data).await
    }

    async fn get_bytes(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<Vec<u8>> {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        http::get_bytes(self.http_client.clone(), &url, params).await
    }
}

impl<Token: data::oauth2token::OAuth2Token + Clone> Clone for HttpClient<Token> {
    fn clone(&self) -> Self {
        HttpClient::<Token> {
            oauth2_token: self.oauth2_token.clone(),
            http_client: self.http_client.clone(),
            secure: self.secure,
        }
    }
}
