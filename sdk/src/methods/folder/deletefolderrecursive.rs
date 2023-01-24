use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::types::FolderID;

pub const ENDPOINT: &str = "/deletefolderrecursive";

pub enum DeleteFolderRecursiveInput {
    FolderID(FolderID),
    Path(PathBuf),
}

impl TryFrom<DeleteFolderRecursiveInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: DeleteFolderRecursiveInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        match value {
            DeleteFolderRecursiveInput::FolderID(fid) => {
                params.insert("folderid".to_string(), fid.0.to_string());
            }
            DeleteFolderRecursiveInput::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFolderRecursive {
    pub deletedfiles: u32,
    pub deletedfolders: u32,
}

#[async_trait]
pub trait GetDeleteFolderRecursive: client::Client {
    async fn deletefolderrecursive(&self, input: DeleteFolderRecursiveInput) -> Result<DeleteFolderRecursive> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<DeleteFolderRecursive>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetDeleteFolderRecursive for T {}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_params_with_fileid() {
        let input = DeleteFolderRecursiveInput::FolderID(FolderID(1234));
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_params_with_path() {
        let input = DeleteFolderRecursiveInput::Path(PathBuf::from("/this/is/the/path"));
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("path"), Some(&"/this/is/the/path".to_string()));
    }

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
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
