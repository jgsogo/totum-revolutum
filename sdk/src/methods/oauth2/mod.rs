use std::net::SocketAddr;

use reqwest;

mod oauth2_token;
mod server;
use crate::data;
pub use oauth2_token::OAuth2Token;

pub async fn authorize_oauth2(
    http_client: reqwest::Client,
    app: data::app_client_data::AppClientData,
    address: SocketAddr,
) -> Result<OAuth2Token, Box<dyn std::error::Error + Send + Sync>> {
    server::serve(http_client, app, address).await
}
