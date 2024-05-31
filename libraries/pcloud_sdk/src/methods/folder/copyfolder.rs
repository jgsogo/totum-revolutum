use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;
use http_utils::AddToParams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::structures::MetadataFolder;
use crate::types::Folder;

pub const ENDPOINT: &str = "/copyfolder";

pub struct CopyFolderInput {
    source: Folder,
    target: Folder,

    /// If it is set and files with the same name already exist, no overwriting will be
    /// preformed and error 2004 will be returned
    noover: bool,

    /// If set will skip files that already exist
    skipexisting: bool,

    /// If it is set only the content of source folder will be copied otherwise the folder
    /// itself is copied
    copycontentonly: bool,
}

impl AddToParams for CopyFolderInput {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        self.source.add_to_params(params);
        match &self.target {
            Folder::FolderID(fid) => {
                params.insert("tofolderid".to_string(), fid.inner().to_string());
            }
            Folder::RemotePath(p) => {
                params.insert("topath".to_string(), p.as_path().to_string());
            }
        }
        if self.noover {
            params.insert("noover".to_string(), "1".to_string());
        }

        if self.skipexisting {
            params.insert("skipexisting".to_string(), "1".to_string());
        }

        if self.copycontentonly {
            params.insert("copycontentonly".to_string(), "1".to_string());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFolder {
    pub metadata: MetadataFolder,
}

#[async_trait]
pub trait GetCopyFolder {
    async fn copyfile(&self, input: CopyFolderInput) -> Result<CopyFolder>;
}

#[async_trait]
impl<T: PCloudClient> GetCopyFolder for T {
    async fn copyfile(&self, input: CopyFolderInput) -> Result<CopyFolder> {
        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &input).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    use camino::Utf8Path;

    use crate::types::FolderID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_params_with_ids() {
        let input = CopyFolderInput {
            source: FolderID::new(1234).into(),
            target: FolderID::new(4321).into(),
            noover: true,
            skipexisting: true,
            copycontentonly: true,
        };
        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        assert_eq!(params.len(), 5);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("noover"), Some(&"1".to_string()));
        assert_eq!(params.get("skipexisting"), Some(&"1".to_string()));
        assert_eq!(params.get("copycontentonly"), Some(&"1".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = CopyFolderInput {
            source: Folder::from_str("path:/source/path").unwrap(),
            target: Folder::from_str("path:/target/path").unwrap(),
            noover: false,
            skipexisting: false,
            copycontentonly: false,
        };
        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/source/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/target/path".to_string()));
    }

    #[test]
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("copyfolder.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<CopyFolder>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.folderid, FolderID::new(230807));
            }
        }
    }
}
