use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

use camino::Utf8Path;
use http::HeaderMap;
use serde::de::DeserializeOwned;

use crate::methods::file::checksumfile;
use crate::methods::file::checksumfile::ChecksumFile;
use crate::methods::folder::{listfolder, ListFolder};
use crate::mocks::client::MockLocalClient;
use crate::mocks::manifest_dir;
use crate::structures::MetadataFile;
use crate::types::FileID;
use crate::utils::http::ApiResult;
use crate::{Error, Result};

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
            assert_eq!(
                params
                    .get("recursive")
                    .ok_or(Error::InputDataError("'recursive' prams not found".to_string()))?,
                "0"
            );

            if let Some(path) = params.get(&"path".to_string()) {
                if path == "/mocked_filesystem" {
                    read_json::<ListFolder>("root.json")
                } else {
                    panic!("No mocked filesystem for path {}", path);
                }
            } else if let Some(folderid) = params.get(&"folderid".to_string()) {
                read_json::<ListFolder>(format!("{}.json", folderid))
            } else {
                panic!("ListFolder mock called with unexpected params {:?}", params);
            }
        });

    client
        .expect_get::<ChecksumFile, _>()
        .withf(
            |endpoint: &str, _headers: &HeaderMap, _params: &HashMap<String, String>| {
                endpoint == checksumfile::ENDPOINT
            },
        )
        .returning(
            move |_endpoint, _headers: HeaderMap, params: &HashMap<String, String>| {
                if let Some(fileid) = params.get(&"fileid".to_string()) {
                    Ok(ChecksumFile {
                        sha1: None,
                        md5: None,
                        sha256: Some(format!("sha256 for fileid {}", fileid)),
                        metadata: MetadataFile {
                            fileid: FileID::new(fileid.parse().unwrap()),
                            common: Default::default(),
                            deletedfileid: None,
                            category: None,
                            hash: None,
                            size: Some(1234),
                            contenttype: None,
                            extra_imagefile: None,
                            extra_audiofile: None,
                            extra_videofile: None,
                        },
                    })
                } else {
                    panic!("No mocked pcloud.checksum for params {:?}", params);
                }
            },
        );

    client
}
