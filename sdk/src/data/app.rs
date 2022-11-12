use serde::{Deserialize, Serialize};

use super::oauth2token::OAuth2Token;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct App {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,

    pub tokens: Vec<OAuth2Token>,
}

impl App {
    pub fn new(name: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            name: name.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn default(client_id: &str, client_secret: &str) -> Self {
        Self {
            name: "no-name".to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            tokens: Vec::new(),
        }
    }
}
