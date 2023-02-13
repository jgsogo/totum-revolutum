use std::net::SocketAddr;

use anyhow::Result;
use reqwest;
use serde::de::DeserializeOwned;

pub use app_client_data::AppClientData;

use crate::data;

mod app_client_data;
mod exchange_oauth2_token;
mod server;

pub async fn authorize_oauth2<Token: data::oauth2token::OAuth2Token + DeserializeOwned + Sync + Send + 'static>(
    http_client: reqwest::Client,
    app: AppClientData,
    address: SocketAddr,
) -> Result<Token> {
    server::serve(http_client, app, address).await
}
