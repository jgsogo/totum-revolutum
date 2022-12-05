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

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_userinfo() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/userinfo");
                assert!(params.is_empty());
                Ok(UserInfo {
                    email: "email".to_string(),
                    emailverified: false,
                    registered: Datetime::default(),
                    premium: false,
                    premiumexpires: Datetime::default(),
                    quota: 5,
                    usedquota: 9,
                    language: "language".to_string(),
                })
            });
        let userinfo = client.userinfo().await?;
        assert_eq!(userinfo.email, "email".to_string());
        assert_eq!(userinfo.emailverified, false);
        assert_eq!(userinfo.registered, Datetime::default());
        assert_eq!(userinfo.premium, false);
        assert_eq!(userinfo.premiumexpires, Datetime::default());
        assert_eq!(userinfo.quota, 5);
        assert_eq!(userinfo.usedquota, 9);
        assert_eq!(userinfo.language, "language".to_string());
        Ok(())
    }
}
