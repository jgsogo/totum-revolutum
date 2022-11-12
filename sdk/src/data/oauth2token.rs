
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OAuth2Token {
    pub userid: i32,
    pub locationid: u8,
    pub access_token: String,
    pub token_type: String,
}
