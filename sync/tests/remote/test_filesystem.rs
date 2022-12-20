use std::env;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::macros::datetime;

use httpmock::prelude::*;
use pcloud_sdk::client::HttpClient;
use pcloud_sdk::data::oauth2token::OAuth2Token;
use pcloud_sdk::methods::general::userinfo::GetUserInfo;
use pcloud_sync::remote::filesystem::FilesystemPCloud;

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

// fn oauth2_token() -> impl OAuth2Token {
//     match env::var("TESTING_PCLOUD_TOKEN") {
//         Ok(v) => {
//             // If the variable is available, then we are using actual pcloud
//         }
//         Err(e) => {
//             // If it is not available, we use a mocked server and a temporal filesystem
//
//             // Start a lightweight mock server.
//             let server = MockServer::start();
//
//             // Create a mock on the server.
//             let userinfo_mock = server.mock(|when, then| {
//                 when.method(GET).path("/userinfo").query_param("access_token", "token");
//
//                 let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//                 let userinfo_json = Path::new(&manifest_dir)
//                     .join("resources")
//                     .join("testdata")
//                     .join("userinfo.json");
//                 then.status(200)
//                     .header("content-type", "application/json; charset=UTF-8")
//                     .body_from_file(userinfo_json.to_str().unwrap());
//             });
//
//             OAuth2TokenMock::new(&format!("{}:{}", server.host(), server.port()), "token")
//         }
//     }
// }

#[tokio::test]
async fn test_create_write_read() -> Result<()> {
    // let oauth2_token = OAuth2TokenMock::new(&format!("{}:{}", server.host(), server.port()), "token");
    // let client = HttpClient::<OAuth2TokenMock>::new(oauth2_token, false);
    //
    // let r = FilesystemPCloud::new(Path::new("the/path"), client).await;

    Ok(())
}
