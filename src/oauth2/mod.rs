use std::net::SocketAddr;

use hyper::client::connect::Connect;

pub mod app_client_data;
pub mod oauth2_token;
mod server;
use reqwest;

pub async fn authorize_oauth2(
    http_client: reqwest::Client,
    app: app_client_data::AppClientData,
    address: SocketAddr,
) -> Result<oauth2_token::OAuth2Token, Box<dyn std::error::Error + Send + Sync>>
{
    server::serve(http_client, app, address).await
}
