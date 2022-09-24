use std::collections::HashMap;
use std::net::SocketAddr;

use async_trait::async_trait;
use hyper;
use hyper::client::connect::Connect;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use crate::oauth2;
use crate::oauth2::app_client_data;
use crate::oauth2::oauth2_token::OAuth2Token;
use crate::utils;

#[async_trait]
pub trait Client<C>: Clone
    where
        C: Connect + Clone + Send + Sync + 'static
{
    fn hostname(&self) -> String;
    fn access_token(&self) -> String;
    fn http_client(&self) -> hyper::Client<C>;

    async fn get<T>(&self, url: &str, mut params: HashMap<String, String>) -> Result<T, hyper::Error>
        where
            T: DeserializeOwned
    {
        params.insert("access_token".to_string(), self.access_token());
        utils::get::<C, T>(self.http_client(), url, params).await
    }

    async fn post<T>(&self, url: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> Result<T, hyper::Error>
        where
            T: DeserializeOwned
    {
        params.insert("access_token".to_string(), self.access_token());
        utils::post::<C, T>(self.http_client(), url, params, data).await
    }
}

pub struct HttpClient {
    oauth2_token: OAuth2Token,
    http_client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl HttpClient {
    fn create_http_client() -> hyper::Client<HttpsConnector<HttpConnector>> {
        let http_client = {
            let https = HttpsConnector::new();
            hyper::Client::builder().build::<_, hyper::Body>(https)
        };
        http_client
    }

    pub fn new(oauth2_token: OAuth2Token) -> HttpClient {
        HttpClient {
            oauth2_token,
            http_client: HttpClient::create_http_client(),
        }
    }

    pub async fn authorize(app: app_client_data::AppClientData, address: SocketAddr) -> Result<HttpClient, Box<dyn std::error::Error + Send + Sync>> {
        let client = HttpClient::create_http_client();
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

impl Client<HttpsConnector<HttpConnector>> for HttpClient {
    fn hostname(&self) -> String {
        self.oauth2_token.hostname()
    }

    fn access_token(&self) -> String {
        self.oauth2_token.access_token.clone()
    }

    fn http_client(&self) -> hyper::Client<HttpsConnector<HttpConnector>> {
        self.http_client.clone()
    }
}
