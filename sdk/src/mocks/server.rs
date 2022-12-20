use std::env;
use std::path::Path;

use httpmock::prelude::*;
use httpmock::Mock;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::data::oauth2token::OAuth2Token;

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

/// Mocks a PCloud server instance to be used in testing. Data is persisted in memory
pub struct PCloudServerMock {
    pub server: MockServer,
}

impl PCloudServerMock {
    pub fn new() -> Self {
        let server = MockServer::start();
        Self { server }
    }

    pub fn token(&self) -> impl OAuth2Token + DeserializeOwned + Sync + Send + 'static {
        OAuth2TokenMock::new(&format!("{}:{}", self.server.host(), self.server.port()), "token")
    }

    pub fn userinfo_mock(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(GET).path("/userinfo").query_param("access_token", "token");

            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let userinfo_json = Path::new(&manifest_dir)
                .join("resources")
                .join("testdata")
                .join("userinfo.json");
            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body_from_file(userinfo_json.to_str().unwrap());
        })
    }
}
