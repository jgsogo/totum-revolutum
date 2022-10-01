use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};

use crate::oauth2::app_client_data::AppClientData;
use crate::utils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OAuth2Token {
    pub result: u16,
    pub userid: i32,
    pub locationid: u8,
    pub access_token: String,
    pub token_type: String,
}

impl OAuth2Token {
    pub fn hostname(&self) -> String {
        match self.locationid {
            1 => "api.pcloud.com".to_string(),
            2 => "eapi.pcloud.com".to_string(),
            _ => panic!("Never"),
        }
    }
}

/*
impl client::Client for OAuth2Token {
    fn hostname(&self) -> String {
        self.hostname()
    }

    fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

 */

pub(crate) async fn exchange_oauth2_token(
    http_client: reqwest::Client,
    app: AppClientData,
    hostname: String,
    code: String,
) -> Result<OAuth2Token, Box<dyn std::error::Error + Send + Sync>> {
    let params = {
        let mut params = HashMap::new();
        params.insert("client_id".to_string(), app.client_id);
        params.insert("client_secret".to_string(), app.client_secret);
        params.insert("code".to_string(), code);
        params
    };

    let url = format!("https://{hostname}/oauth2_token");
    let oauth2_token = utils::get::<OAuth2Token>(http_client, &url, params).await?;
    Ok(oauth2_token)
}
