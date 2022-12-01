use std::collections::HashMap;

use anyhow::Result;
use reqwest;

use crate::data;
use crate::data::app_client_data::AppClientData;
use crate::utils;

pub(crate) async fn exchange_oauth2_token(
    http_client: reqwest::Client,
    app: AppClientData,
    hostname: String,
    code: String,
) -> Result<data::oauth2token::OAuth2Token> {
    let params = {
        let mut params = HashMap::new();
        params.insert("client_id".to_string(), app.client_id);
        params.insert("client_secret".to_string(), app.client_secret);
        params.insert("code".to_string(), code);
        params
    };

    let url = format!("https://{hostname}/oauth2_token");
    let oauth2_token = utils::http::get::<data::oauth2token::OAuth2Token>(http_client, &url, params).await?;
    Ok(oauth2_token)
}
