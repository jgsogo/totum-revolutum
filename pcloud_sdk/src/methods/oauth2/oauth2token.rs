use serde::{Deserialize, Serialize};

use crate::access_token::OAuth2Token;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OAuth2TokenImpl {
    pub userid: i32,
    pub locationid: u8,
    pub access_token: String,
    pub token_type: String,
}

impl OAuth2TokenImpl {
    pub fn userid(&self) -> i32 {
        self.userid
    }
}

impl OAuth2Token for OAuth2TokenImpl {
    fn hostname(&self) -> String {
        match self.locationid {
            1 => "api.pcloud.com".to_string(),
            2 => "eapi.pcloud.com".to_string(),
            _ => panic!("Never"),
        }
    }

    fn access_token(&self) -> &str {
        &self.access_token
    }
}
