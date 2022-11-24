use std::collections::HashMap;
use std::net::SocketAddr;

use super::data;
use crate::methods::oauth2;
use crate::utils::http;
use async_trait::async_trait;
use reqwest;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait Client: Clone {
    fn hostname(&self) -> String;
    fn access_token(&self) -> String;
    fn http_client(&self) -> reqwest::Client;

    async fn get<T>(
        &self,
        url: &str,
        mut params: HashMap<String, String>,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        T: DeserializeOwned,
    {
        params.insert("access_token".to_string(), self.access_token());
        http::get::<T>(self.http_client(), url, params).await
    }

    async fn post<T>(
        &self,
        url: &str,
        mut params: HashMap<String, String>,
        data: Vec<u8>,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        T: DeserializeOwned,
    {
        params.insert("access_token".to_string(), self.access_token());
        http::post::<T>(self.http_client(), url, params, data).await
    }
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

    pub async fn authorize(
        app: data::app_client_data::AppClientData,
        address: SocketAddr,
    ) -> Result<HttpClient, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let oauth2 = oauth2::authorize_oauth2(client.clone(), app, address).await?;
        Ok(HttpClient {
            oauth2_token: oauth2,
            http_client: client,
        })
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

impl Client for HttpClient {
    fn hostname(&self) -> String {
        self.oauth2_token.hostname()
    }

    fn access_token(&self) -> String {
        self.oauth2_token.access_token.clone()
    }

    fn http_client(&self) -> reqwest::Client {
        self.http_client.clone()
    }
}
