use crate::client::PCloudClient;
use crate::Result;
use async_trait::async_trait;
use http::HeaderMap;
use http_utils::rest::RESTClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    // userid
    pub userid: u64,
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

impl UserInfo {
    pub fn quota_in_gigas(&self) -> u64 {
        self.quota / 1e9 as u64
    }

    pub fn usedquota_in_gigas(&self) -> u64 {
        self.usedquota / 1e9 as u64
    }
}

#[async_trait]
pub trait GetUserInfo {
    async fn userinfo(&self) -> Result<UserInfo>;
}

#[async_trait]
impl<T: PCloudClient> GetUserInfo for T
where
    T: http_utils::HttpClient<Error = crate::Error>,
{
    async fn userinfo(&self) -> Result<UserInfo> {
        RESTClient::get(self, "/userinfo", HeaderMap::default(), &HashMap::new()).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;
    use time::macros::datetime;

    use crate::mocks::client::MockLocalClient;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_deserialize_userinfo() {
        fn reader() -> BufReader<std::fs::File> {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let userinfo_json = Utf8Path::new(&manifest_dir)
                .join("resources")
                .join("testdata")
                .join("userinfo.json");
            let file = File::open(userinfo_json).unwrap();
            BufReader::new(file)
        }

        // Deserialize with ApiResult wrapper
        match serde_json::from_reader::<_, ApiResult<UserInfo>>(reader()) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                assert_eq!(data.error, None);
                let data = data.data.unwrap();
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
            .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, "/userinfo");
                assert!(params.is_empty());
                assert_eq!(headers.len(), 0);
                Ok(UserInfo {
                    userid: 1234,
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
        assert_eq!(userinfo.userid, 1234);
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
