use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::FolderID;

pub const ENDPOINT: &str = "/renamefolder";

pub enum RenameFolderLocation {
    FolderID(FolderID),
    Path(PathBuf),
}

pub struct RenameFolderTargetID {
    folderid: Option<FolderID>,
    name: Option<String>,
}

impl RenameFolderTargetID {
    pub fn new(folderid: Option<FolderID>, name: Option<String>) -> Result<Self> {
        if folderid.is_none() && name.is_none() {
            bail!("folderid or name (or both) are required");
        }
        Ok(Self { folderid, name })
    }
}

pub enum RenameFolderTargetInput {
    FolderID(RenameFolderTargetID),
    Path(PathBuf),
}

pub struct RenameFolderInput {
    source: RenameFolderLocation,
    target: RenameFolderTargetInput,
}

impl TryFrom<RenameFolderInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: RenameFolderInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        let RenameFolderInput { source, target } = value;
        match source {
            RenameFolderLocation::FolderID(fid) => {
                params.insert("folderid".to_string(), fid.0.to_string());
            }
            RenameFolderLocation::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        match target {
            RenameFolderTargetInput::FolderID(target) => {
                if let Some(folderid) = target.folderid {
                    params.insert("tofolderid".to_string(), folderid.0.to_string());
                }
                if let Some(name) = target.name {
                    params.insert("toname".to_string(), name);
                }
            }
            RenameFolderTargetInput::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameFolder {
    pub metadata: Metadata,
}

/// Rename or move
#[async_trait]
pub trait GetRenameFolder: client::Client {
    async fn copyfile(&self, input: RenameFolderInput) -> Result<RenameFolder> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<RenameFolder>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetRenameFolder for T {}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_params_with_ids_noname() {
        let input = RenameFolderInput {
            source: RenameFolderLocation::FolderID(FolderID(1234)),
            target: RenameFolderTargetInput::FolderID(RenameFolderTargetID::new(Some(FolderID(4321)), None).unwrap()),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = RenameFolderInput {
            source: RenameFolderLocation::FolderID(FolderID(1234)),
            target: RenameFolderTargetInput::FolderID(
                RenameFolderTargetID::new(Some(FolderID(4321)), Some("name".to_string())).unwrap(),
            ),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_ids_only_name() {
        let input = RenameFolderInput {
            source: RenameFolderLocation::FolderID(FolderID(1234)),
            target: RenameFolderTargetInput::FolderID(
                RenameFolderTargetID::new(None, Some("name".to_string())).unwrap(),
            ),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_ids_invalid() {
        assert!(RenameFolderTargetID::new(None, None).is_err());
    }

    #[test]
    fn test_params_with_paths() {
        let input = RenameFolderInput {
            source: RenameFolderLocation::Path(PathBuf::from("/from/path")),
            target: RenameFolderTargetInput::Path(PathBuf::from("/to/path")),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
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
