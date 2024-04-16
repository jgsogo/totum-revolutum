use std::collections::HashMap;

use anyhow::Result;
use reqwest;
use serde::de::DeserializeOwned;

use crate::utils;

use super::{AppClientData, OAuth2Token};

/// This method is used when an app is using the code flow. The app calls this method
/// to obtain a bearer token, after the user had authorized the app.
pub(crate) async fn exchange_oauth2_token<Token: OAuth2Token + DeserializeOwned>(
    http_client: reqwest::Client,
    app: AppClientData,
    hostname: String,
    code: String,
) -> Result<Token> {
    let params = {
        let mut params = HashMap::new();
        params.insert("client_id".to_string(), app.client_id().to_string());
        params.insert("client_secret".to_string(), app.client_secret().to_string());
        params.insert("code".to_string(), code);
        params
    };

    let url = format!("https://{hostname}/oauth2_token");
    let oauth2_token = utils::http::get::<Token>(http_client, &url, params).await?;
    Ok(oauth2_token)
}
