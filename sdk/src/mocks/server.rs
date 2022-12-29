use std::collections::HashMap;
use std::env;

use std::path::Path;

use httpmock::prelude::*;
use httpmock::Mock;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::data::oauth2token::OAuth2Token;
use crate::methods::fileops::{file_close, file_open, file_read, file_write};
use crate::methods::folder::listfolder;

use crate::types::FolderID;

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
    fd_count: u64,
}

impl Default for PCloudServerMock {
    fn default() -> Self {
        let server = MockServer::start();
        Self { server, fd_count: 0 }
    }
}

impl PCloudServerMock {
    pub fn token(&self) -> impl OAuth2Token + DeserializeOwned + Sync + Send + Clone + 'static {
        OAuth2TokenMock::new(&format!("{}:{}", self.server.host(), self.server.port()), "token")
    }

    pub fn listfolder_mock(&self, params: HashMap<&str, &str>, body: impl AsRef<[u8]>) -> Mock {
        self.server.mock(|when, then| {
            let mut wh = when.method(GET).path(listfolder::ENDPOINT);

            for (k, v) in params {
                wh = wh.query_param(k, v);
            }

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body(body);
        })
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

    pub fn fileops_create_with_folder_and_name(
        &mut self,
        folder: FolderID,
        name: &str,
        write_bytes: u64,
        read_content: Vec<u8>,
    ) -> (Mock, Mock, Mock, Mock, Mock) {
        self.fd_count += 1;
        let fd = self.fd_count;
        let fileid = folder.0 + fd;

        // Enable file_open with folderid+path
        let m1 = self.server.mock(|when, then| {
            when.method(GET)
                .path(file_open::ENDPOINT)
                .query_param("access_token", "token")
                .query_param("folderid", folder.0.to_string())
                .query_param("name", name)
                .query_param_exists("flags");

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body(format!("{{\"result\": 0, \"fd\": {fd}, \"fileid\": {fileid} }}"));
        });

        // Enable file_open with fileid
        let m2 = self.server.mock(|when, then| {
            when.method(GET)
                .path(file_open::ENDPOINT)
                .query_param("access_token", "token")
                .query_param("fileid", fileid.to_string())
                .query_param_exists("flags");

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body(format!("{{\"result\": 0, \"fd\": {fd}, \"fileid\": {fileid} }}"));
        });

        // Enable file_write
        let m3 = self.server.mock(|when, then| {
            when.method(POST)
                .path(file_write::ENDPOINT)
                .query_param("access_token", "token")
                .query_param("fd", fd.to_string());

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body(format!("{{\"result\": 0, \"bytes\": {write_bytes} }}"));
        });

        // Enable file_read
        let m4 = self.server.mock(|when, then| {
            when.method(GET)
                .path(file_read::ENDPOINT)
                .query_param("access_token", "token")
                .query_param("fd", fd.to_string())
                .query_param_exists("count");

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body(read_content);
        });

        // Enable file_close
        let m5 = self.server.mock(|when, then| {
            when.method(GET)
                .path(file_close::ENDPOINT)
                .query_param("access_token", "token")
                .query_param("fd", fd.to_string());

            then.status(200)
                .header("content-type", "application/json; charset=UTF-8")
                .body("{\"result\": 0 }");
        });

        (m1, m2, m3, m4, m5)
    }
}
