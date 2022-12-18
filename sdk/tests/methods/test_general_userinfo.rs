use std::env;
use std::path::Path;

use anyhow::Result;
use httpmock::prelude::*;
use serde::{Deserialize, Serialize};
use time::macros::datetime;

use pcloud_sdk::client::HttpClient;
use pcloud_sdk::data::oauth2token::OAuth2Token;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct OAuth2TokenMock {
    hostname: String,
    access_token: String,
}

impl OAuth2TokenMock {
    pub fn new(hostname: &str, access_token: &str) -> Self {
        Self {
            hostname: hostname.to_string(),
            access_token: access_token.to_string(),
        }
    }
}

impl OAuth2Token for OAuth2TokenMock {
    fn hostname(&self) -> String {
        self.hostname.clone()
    }

    fn access_token(&self) -> &str {
        &self.access_token
    }
}

#[tokio::test]
async fn test_userinfo_get() -> Result<()> {
    // Start a lightweight mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let userinfo_mock = server.mock(|when, then| {
        when.method(GET).path("/userinfo").query_param("access_token", "token");

        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("userinfo.json");
        then.status(200)
            .header("content-type", "application/json; charset=UTF-8")
            .body_from_file(userinfo_json.to_str().unwrap());
    });
    let oauth2_token = OAuth2TokenMock::new(&format!("{}:{}", server.host(), server.port()), "token");
    let pcloud = HttpClient::<OAuth2TokenMock>::new(oauth2_token, false);
    let data = pcloud.userinfo().await?;

    assert_eq!(data.email, "pcloud@pcloud.com".to_string());
    assert_eq!(data.emailverified, true);
    assert_eq!(data.registered, datetime!(2013-11-18 15:32:05 UTC));
    assert_eq!(data.premium, false);
    assert_eq!(data.premiumexpires, None);
    assert_eq!(data.quota, 1000);
    assert_eq!(data.usedquota, 500);
    assert_eq!(data.language, "en".to_string());

    userinfo_mock.assert();
    Ok(())
}
