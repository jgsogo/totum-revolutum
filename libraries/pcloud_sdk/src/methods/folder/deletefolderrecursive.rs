use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::params::Params;
use crate::types::Folder;

pub const ENDPOINT: &str = "/deletefolderrecursive";

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFolderRecursive {
    pub deletedfiles: u32,
    pub deletedfolders: u32,
}

#[async_trait]
pub trait GetDeleteFolderRecursive {
    async fn deletefolderrecursive(&self, input: Folder) -> Result<DeleteFolderRecursive>;
}

#[async_trait]
impl<T: PCloudClient> GetDeleteFolderRecursive for T {
    async fn deletefolderrecursive(&self, input: Folder) -> Result<DeleteFolderRecursive> {
        let ret = RESTClient::get::<DeleteFolderRecursive>(self, ENDPOINT, HeaderMap::default(), input.into_params()?)
            .await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;

    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("deletefolderrecursive.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<DeleteFolderRecursive>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.deletedfiles, 30);
                assert_eq!(data.deletedfolders, 5);
            }
        }
    }
}
