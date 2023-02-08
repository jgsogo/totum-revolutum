use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::FolderID;

pub const ENDPOINT: &str = "/copyfolder";

pub enum CopyFolderLocation {
    FolderID(FolderID),
    Path(PathBuf),
}

pub struct CopyFolderInput {
    source: CopyFolderLocation,
    target: CopyFolderLocation,

    /// If it is set and files with the same name already exist, no overwriting will be
    /// preformed and error 2004 will be returned
    noover: bool,

    /// If set will skip files that already exist
    skipexisting: bool,

    /// If it is set only the content of source folder will be copied otherwise the folder
    /// itself is copied
    copycontentonly: bool,
}

impl TryFrom<CopyFolderInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: CopyFolderInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        let CopyFolderInput { source, target, .. } = value;
        match source {
            CopyFolderLocation::FolderID(fid) => {
                params.insert("folderid".to_string(), fid.0.to_string());
            }
            CopyFolderLocation::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        match target {
            CopyFolderLocation::FolderID(fid) => {
                params.insert("tofolderid".to_string(), fid.0.to_string());
            }
            CopyFolderLocation::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        };

        if value.noover {
            params.insert("noover".to_string(), "1".to_string());
        }

        if value.skipexisting {
            params.insert("skipexisting".to_string(), "1".to_string());
        }

        if value.copycontentonly {
            params.insert("copycontentonly".to_string(), "1".to_string());
        }

        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFolder {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetCopyFolder: client::Client {
    async fn copyfile(&self, input: CopyFolderInput) -> Result<CopyFolder> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<CopyFolder>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetCopyFolder for T {}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_params_with_ids() {
        let input = CopyFolderInput {
            source: CopyFolderLocation::FolderID(FolderID(1234)),
            target: CopyFolderLocation::FolderID(FolderID(4321)),
            noover: true,
            skipexisting: true,
            copycontentonly: true,
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
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
            source: CopyFolderLocation::Path(PathBuf::from("/source/path")),
            target: CopyFolderLocation::Path(PathBuf::from("/target/path")),
            noover: false,
            skipexisting: false,
            copycontentonly: false,
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/source/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/target/path".to_string()));
    }

    #[test]
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
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
                assert_eq!(data.metadata.folderid.unwrap(), FolderID(230807));
            }
        }
    }
}
