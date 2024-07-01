use std::collections::HashMap;
use std::net::SocketAddr;

use async_trait::async_trait;
use headers::HeaderMapExt;
use headers::{Header, HeaderMap};
use reqwest;
use serde::de::DeserializeOwned;

use utils::http::rest::RESTClient;
use utils::http::{AddToParams, HttpClient};

use crate::methods::oauth2;
use crate::utils::http;
use crate::utils::http::create_response;
use crate::{access_token, Error, Result};

#[async_trait]
pub trait PCloudClient: RESTClient + HttpClient<Error = Error> {
    async fn get_bytes<TParams: AddToParams + Sync + 'static>(
        &self,
        endpoint: &str,
        params: &TParams,
    ) -> Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct PCloudClientImpl<Token: access_token::OAuth2Token> {
    pub oauth2_token: Token,
    http_client: reqwest::Client,
    secure: bool,
}

impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> PCloudClientImpl<Token> {
    pub fn new(oauth2_token: Token, secure: bool) -> PCloudClientImpl<Token> {
        let client = reqwest::ClientBuilder::new().build().unwrap();
        PCloudClientImpl {
            oauth2_token,
            http_client: client,
            secure,
        }
    }

    pub async fn authorize(app: oauth2::AppClientData, address: SocketAddr) -> Result<PCloudClientImpl<Token>> {
        let client = reqwest::Client::new();
        let oauth2 = oauth2::authorize_oauth2(client.clone(), app, address).await?;
        Ok(PCloudClientImpl {
            oauth2_token: oauth2,
            http_client: client,
            secure: true,
        })
    }
}

#[async_trait]
impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> HttpClient
    for PCloudClientImpl<Token>
{
    type Error = Error;

    fn build_url(&self, endpoint: &str) -> String {
        let schema = if self.secure { "https" } else { "http" };
        format!("{}://{}{}", schema, self.oauth2_token.hostname(), endpoint)
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    /// Appends common headers: if no `Connection` is already there, it will add `Keep-Alive` one.
    fn headers(&self, mut headers: HeaderMap) -> HeaderMap {
        // FIXME: Probably we should remove this method and each call should set its own headers,
        // FIXME: AFAIK, only the `pcloud::fileops` ones require to keep the connection open
        if !headers.contains_key(headers::Connection::name()) {
            let conn = headers::Connection::keep_alive();
            headers.typed_insert(conn);
        }

        headers
    }

    fn params(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        let access_token = self.oauth2_token.access_token();
        params.insert("access_token".to_string(), access_token.to_string());
        params
    }
}

#[async_trait]
impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> PCloudClient
    for PCloudClientImpl<Token>
{
    async fn get_bytes<TParams: AddToParams + Sync>(&self, endpoint: &str, query: &TParams) -> Result<Vec<u8>> {
        let url = self.build_url(endpoint);

        let mut params = self.params();
        query.add_to_params(&mut params);

        http::get_bytes(self.http_client.clone(), &url, params).await
    }
}

impl<Token: access_token::OAuth2Token + DeserializeOwned + Sync + Send + 'static> RESTClient
    for PCloudClientImpl<Token>
{
    fn parse_response<T>(result: String) -> std::result::Result<T, Self::Error>
    where
        T: DeserializeOwned + 'static,
    {
        create_response(result)
    }
}

impl<Token: access_token::OAuth2Token + Clone> Clone for PCloudClientImpl<Token> {
    fn clone(&self) -> Self {
        PCloudClientImpl::<Token> {
            oauth2_token: self.oauth2_token.clone(),
            http_client: self.http_client.clone(),
            secure: self.secure,
        }
    }
}
