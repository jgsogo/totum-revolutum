use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::client;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    // email address of the user
    pub email: String,
    // true if the user had verified it's email
    pub emailverified: bool,
    // when the user was registerd
    #[serde(with = "time::serde::rfc2822")]
    pub registered: OffsetDateTime,
    // true if the user is premium
    pub premium: bool,
    // if premium is true: premiumexpires will be the date until the service is
    #[serde(with = "time::serde::rfc2822::option", default)]
    pub premiumexpires: Option<OffsetDateTime>,
    // in bytes
    pub quota: u64,
    // in bytes, so quite big numbers
    pub usedquota: u64,
    // 2-3 characters lowercase languageid
    pub language: String,
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
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use time::macros::datetime;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_userinfo() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("userinfo.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, UserInfo>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.email, "pcloud@pcloud.com".to_string());
                assert_eq!(data.emailverified, true);
                assert_eq!(data.registered, datetime!(2013-11-18 15:32:05 UTC));
                assert_eq!(data.premium, false);
                assert_eq!(data.premiumexpires, None);
                assert_eq!(data.quota, 1000);
                assert_eq!(data.usedquota, 500);
                assert_eq!(data.language, "en".to_string());
            }
        }
    }

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
                    registered: datetime!(2013-10-02 14:29:11 UTC),
                    premium: false,
                    premiumexpires: None,
                    quota: 5,
                    usedquota: 9,
                    language: "language".to_string(),
                })
            });
        let userinfo = client.userinfo().await?;
        assert_eq!(userinfo.email, "email".to_string());
        assert_eq!(userinfo.emailverified, false);
        assert_eq!(userinfo.registered, datetime!(2013-10-02 14:29:11 UTC));
        assert_eq!(userinfo.premium, false);
        assert_eq!(userinfo.premiumexpires, None);
        assert_eq!(userinfo.quota, 5);
        assert_eq!(userinfo.usedquota, 9);
        assert_eq!(userinfo.language, "language".to_string());
        Ok(())
    }
}
