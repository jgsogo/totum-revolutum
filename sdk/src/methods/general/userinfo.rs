use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;

type Datetime = String; // TODO: Parse actual date

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    // email address of the user
    email: String,
    // true if the user had verified it's email
    emailverified: bool,
    // when the user was registerd
    registered: Datetime,
    // true if the user is premium
    premium: bool,
    // if premium is true: premiumexpires will be the date until the service is
    premiumexpires: Datetime,
    // in bytes
    quota: u64,
    // in bytes, so quite big numbers
    usedquota: u64,
    // 2-3 characters lowercase languageid
    language: String,
}

#[async_trait]
pub trait GetUserInfo: client::Client {
    async fn userinfo(&self) -> Result<UserInfo> {
        self.get::<UserInfo>("/userinfo", HashMap::new()).await
    }
}

impl<T: client::Client> GetUserInfo for T {}
