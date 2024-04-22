use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Result;
use async_trait::async_trait;
use headers::HeaderMap;
use headers::HeaderMapExt;
use reqwest;
use serde::de::DeserializeOwned;

use http_utils::rest::RESTClient;
use http_utils::HttpClient;

use crate::access_token;
use crate::methods::oauth2;
use crate::utils::http;
use crate::utils::http::create_response;

#[async_trait]
pub trait ClientBytes: HttpClient {
    // FIXME: Remove and make it a member function
    async fn get_bytes(&self, endpoint: &str, params: HashMap<String, String>) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct PCloudClient<Token: access_token::OAuth2Token> {
    pub oauth2_token: Token,
    http_client: reqwest::Client,
    secure: bool,
}

impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> PCloudClient<Token> {
    pub fn new(oauth2_token: Token, secure: bool) -> PCloudClient<Token> {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        PCloudClient {
            oauth2_token,
            http_client: client,
            secure,
        }
    }

    pub async fn authorize(app: oauth2::AppClientData, address: SocketAddr) -> Result<PCloudClient<Token>> {
        let client = reqwest::Client::new();
        let oauth2 = oauth2::authorize_oauth2(client.clone(), app, address).await?;
        Ok(PCloudClient {
            oauth2_token: oauth2,
            http_client: client,
            secure: true,
        })
    }
}

#[async_trait]
impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> HttpClient for PCloudClient<Token> {
    fn build_url(&self, endpoint: &str) -> String {
        let schema = if self.secure { "https" } else { "http" };
        format!("{}://{}{}", schema, self.oauth2_token.hostname(), endpoint)
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    fn headers(&self, mut headers: HeaderMap) -> HeaderMap {
        let conn = headers::Connection::keep_alive();
        headers.typed_insert(conn);
        headers
    }

    fn params(&self, mut params: HashMap<String, String>) -> HashMap<String, String> {
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        params
    }
}

#[async_trait]
impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> ClientBytes for PCloudClient<Token> {
    async fn get_bytes(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<Vec<u8>> {
        let url = self.build_url(endpoint);
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        http::get_bytes(self.http_client.clone(), &url, params).await
    }
}

impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> RESTClient for PCloudClient<Token> {
    fn parse_response<T>(result: String) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        create_response(result)
    }
}

impl<Token: access_token::OAuth2Token + Clone> Clone for PCloudClient<Token> {
    fn clone(&self) -> Self {
        PCloudClient::<Token> {
            oauth2_token: self.oauth2_token.clone(),
            http_client: self.http_client.clone(),
            secure: self.secure,
        }
    }
}
