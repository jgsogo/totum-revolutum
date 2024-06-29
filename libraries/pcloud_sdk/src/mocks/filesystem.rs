use std::collections::HashMap;
use std::io::BufReader;
use std::str::FromStr;
use std::{env, fs};

use camino::Utf8Path;
use http::HeaderMap;
use serde::de::DeserializeOwned;

use crate::methods::folder::{listfolder, ListFolder};
use crate::mocks::client::MockLocalClient;
use crate::utils::http::ApiResult;
use crate::Result;

pub fn read_json<T: DeserializeOwned>(p: impl AsRef<Utf8Path>) -> Result<T> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let filepath_json = Utf8Path::new(&manifest_dir)
        .join("resources")
        .join("mocked_filesystem")
        .join(p);
    let file = fs::File::open(&filepath_json).unwrap();
    let reader = BufReader::new(file);
    let r = serde_json::from_reader::<_, ApiResult<T>>(reader).unwrap();
    Ok(r.data.unwrap())
}

/// Returns a [`MockLocalClient`] prepared to execute [`listfolder::GetListFolder::listfolder`] endpoint.
pub fn filesystem_mocked() -> MockLocalClient {
    let mut client = MockLocalClient::new();

    client
        .expect_get::<ListFolder, _>()
        .withf(
            |endpoint: &str, _headers: &HeaderMap, _params: &HashMap<String, String>| endpoint == listfolder::ENDPOINT,
        )
        .returning(move |_endpoint, _headers: HeaderMap, params: &HashMap<_, _>| {
            if let Some(path) = params.get(&"path".to_string()) {
                if path == "/mocked_filesystem" {
                    read_json::<ListFolder>("root.json")
                } else {
                    panic!("No mocked filesystem for path {}", path);
                }
            } else if let Some(folderid) = params.get(&"folderid".to_string()) {
                let folderid: u64 = u64::from_str(folderid).unwrap();
                if folderid == 11801912705 {
                    read_json::<ListFolder>("root.json")
                } else {
                    panic!("No mocked filesystem for folderid {}", folderid);
                }
            } else {
                panic!("ListFolder called with invalid params {:?}", params);
            }
        });

    client
}
