use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use anyhow::Result;
use httpmock::prelude::*;
use serde::{Deserialize, Serialize};

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
        let mut file = File::open(&userinfo_json).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect(&format!("Cannot read file {}", userinfo_json.display()));
        then.status(200)
            .header("content-type", "application/json; charset=UTF-8")
            .body_from_file(userinfo_json.to_str().unwrap());
    });

    let r = server.url("/userinfo?access_token=token");
    let response = isahc::get(server.url("/userinfo?access_token=token")).unwrap();
    let response = isahc::get(server.url("/userinfo?access_token=token")).unwrap();
    assert_eq!(response.status(), 200);
    // assert_eq!(response.text()?, "lol");

    let oauth2_token = OAuth2TokenMock::new(&format!("{}:{}", server.host(), server.port()), "token");
    let pcloud = HttpClient::<OAuth2TokenMock>::new(oauth2_token);
    let _r = pcloud.userinfo().await?;

    userinfo_mock.assert();
    Ok(())
}
