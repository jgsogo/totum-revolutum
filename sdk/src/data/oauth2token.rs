use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OAuth2Token {
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
