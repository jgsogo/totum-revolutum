use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::{Params, SourceAndTargetFolder};
use crate::structures::Metadata;

pub const ENDPOINT: &str = "/renamefolder";

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameFolder {
    pub metadata: Metadata,
}

/// Rename or move
#[async_trait]
pub trait GetRenameFolder {
    async fn copyfile(&self, input: SourceAndTargetFolder) -> Result<RenameFolder>;
}

#[async_trait]
impl<T: client::Client> GetRenameFolder for T {
    async fn copyfile(&self, input: SourceAndTargetFolder) -> Result<RenameFolder> {
        let ret = self.get::<RenameFolder>(ENDPOINT, input.into_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use std::path::PathBuf;
    use std::str::FromStr;

    use crate::methods::params::TargetLocation;
    use crate::types::{Folder, FolderID};
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_params_with_ids_noname() {
        let input = SourceAndTargetFolder {
            source: Folder::FolderID(FolderID(1234)),
            target: TargetLocation::FolderAndName((FolderID(4321), None)),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = SourceAndTargetFolder {
            source: Folder::FolderID(FolderID(1234)),
            target: TargetLocation::FolderAndName((FolderID(4321), Some("name".to_string()))),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = SourceAndTargetFolder {
            source: Folder::from_str("/from/path").unwrap(),
            target: TargetLocation::Path(PathBuf::from("/to/path")),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/from/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/to/path".to_string()));
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
        match serde_json::from_reader::<_, ApiResult<RenameFolder>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.folderid.unwrap(), FolderID(230807));
            }
        }
    }
}
