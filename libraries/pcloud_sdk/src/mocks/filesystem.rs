use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::str::FromStr;

use camino::Utf8Path;
use http::HeaderMap;
use serde::de::DeserializeOwned;

use crate::methods::folder::{listfolder, ListFolder};
use crate::mocks::client::MockLocalClient;
use crate::mocks::manifest_dir;
use crate::utils::http::ApiResult;
use crate::Result;

pub fn read_json<T: DeserializeOwned>(p: impl AsRef<Utf8Path>) -> Result<T> {
    let filepath_json = manifest_dir().join("resources").join("mocked_filesystem").join(p);
    let file = match fs::File::open(&filepath_json) {
        Ok(file) => file,
        Err(e) => {
            panic!("Failed to open file {}: {}", filepath_json, e)
        }
    };
    let reader = BufReader::new(file);
    let r = serde_json::from_reader::<_, ApiResult<T>>(reader).unwrap();
    Ok(r.data.unwrap())
}

/// Returns a [`MockLocalClient`] prepared to execute [`listfolder::GetListFolder::listfolder`] endpoint.
///
/// # Examples
///
/// ```rust
/// use std::str::FromStr;
/// use pcloud_sdk::methods::folder::listfolder::GetListFolder;
/// use pcloud_sdk::methods::folder::ListFolderInput;
/// use pcloud_sdk::mocks::filesystem::filesystem_mocked;
/// use pcloud_sdk::types::{Folder, RemotePath};
///
/// # #[tokio::main]
/// # async fn main() {
/// let client = filesystem_mocked();
/// let folder = Folder::from(RemotePath::from_str("path:/mocked_filesystem").unwrap());
/// let input = ListFolderInput::new(folder);
/// let data = client.listfolder(input).await;
/// # }
/// ```
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
