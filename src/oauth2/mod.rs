use std::net::SocketAddr;

use hyper::client::connect::Connect;

mod server;
pub mod oauth2_token;
pub mod app_client_data;


pub async fn authorize_oauth2<C>(http_client: hyper::Client<C>, app: app_client_data::AppClientData, address: SocketAddr) -> Result<oauth2_token::OAuth2Token, Box<dyn std::error::Error + Send + Sync>>
    where
        C: Connect + Clone + Send + Sync + 'static
{
    server::serve(http_client, app, address).await
}
