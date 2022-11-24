use anyhow::Result;
use reqwest;
use std::net::SocketAddr;

mod oauth2_token;
mod server;
use crate::data;

pub async fn authorize_oauth2(
    http_client: reqwest::Client,
    app: data::app_client_data::AppClientData,
    address: SocketAddr,
) -> Result<data::oauth2token::OAuth2Token> {
    server::serve(http_client, app, address).await
}
