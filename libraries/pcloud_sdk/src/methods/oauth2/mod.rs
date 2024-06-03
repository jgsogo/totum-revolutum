use std::net::SocketAddr;

use reqwest;
use serde::de::DeserializeOwned;

pub use app_client_data::AppClientData;
pub use oauth2token::OAuth2TokenImpl;

use crate::access_token::OAuth2Token;
use crate::Result;

mod app_client_data;
mod exchange_oauth2_token;
mod oauth2token;
mod server;

pub async fn authorize_oauth2<Token: OAuth2Token + DeserializeOwned + Sync + Send + 'static>(
    http_client: reqwest::Client,
    app: AppClientData,
    address: SocketAddr,
) -> Result<Token> {
    server::serve(http_client, app, address).await
}
