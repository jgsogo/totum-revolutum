use std::net::SocketAddr;

use anyhow::Result;
use reqwest;
use serde::de::DeserializeOwned;

use crate::data;

mod oauth2_token;
mod server;

pub async fn authorize_oauth2<Token: data::oauth2token::OAuth2Token + DeserializeOwned + Sync + Send + 'static>(
    http_client: reqwest::Client,
    app: data::app_client_data::AppClientData,
    address: SocketAddr,
) -> Result<Token> {
    server::serve(http_client, app, address).await
}
