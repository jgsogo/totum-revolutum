use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{bail, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::{FileID, FolderID};

pub const ENDPOINT: &str = "/renamefile";

pub enum RenameFileSourceInput {
    FileID(FileID),
    Path(PathBuf),
}

pub struct RenameFileTargetID {
    folderid: Option<FolderID>,
    name: Option<String>,
}

impl RenameFileTargetID {
    pub fn new(folderid: Option<FolderID>, name: Option<String>) -> Result<Self> {
        if folderid.is_none() && name.is_none() {
            bail!("folderid or name (or both) are required");
        }
        Ok(Self { folderid, name })
    }
}

pub enum RenameFileTargetInput {
    FolderID(RenameFileTargetID),
    Path(PathBuf),
}

pub struct RenameFileInput {
    source: RenameFileSourceInput,
    target: RenameFileTargetInput,
}

impl TryFrom<RenameFileInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: RenameFileInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        let RenameFileInput { source, target } = value;
        match source {
            RenameFileSourceInput::FileID(fid) => {
                params.insert("fileid".to_string(), fid.0.to_string());
            }
            RenameFileSourceInput::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        match target {
            RenameFileTargetInput::FolderID(target) => {
                if let Some(folderid) = target.folderid {
                    params.insert("tofolderid".to_string(), folderid.0.to_string());
                }
                if let Some(name) = target.name {
                    params.insert("toname".to_string(), name);
                }
            }
            RenameFileTargetInput::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameFile {
    pub metadata: Metadata,
}

/// Renames or moves
#[async_trait]
pub trait GetRenameFile: client::Client {
    async fn renamefile(&self, input: RenameFileInput) -> Result<RenameFile> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<RenameFile>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetRenameFile for T {}

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
        let input = RenameFileInput {
            source: RenameFileSourceInput::FileID(FileID(1234)),
            target: RenameFileTargetInput::FolderID(RenameFileTargetID::new(Some(FolderID(4321)), None).unwrap()),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = RenameFileInput {
            source: RenameFileSourceInput::FileID(FileID(1234)),
            target: RenameFileTargetInput::FolderID(
                RenameFileTargetID::new(Some(FolderID(4321)), Some("name".to_string())).unwrap(),
            ),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_ids_only_name() {
        let input = RenameFileInput {
            source: RenameFileSourceInput::FileID(FileID(1234)),
            target: RenameFileTargetInput::FolderID(RenameFileTargetID::new(None, Some("name".to_string())).unwrap()),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_ids_invalid() {
        assert!(RenameFileTargetID::new(None, None).is_err());
    }

    #[test]
    fn test_params_with_paths() {
        let input = RenameFileInput {
            source: RenameFileSourceInput::Path(PathBuf::from("/from/path")),
            target: RenameFileTargetInput::Path(PathBuf::from("/to/path")),
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
            .join("renamefile.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<RenameFile>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.fileid.unwrap(), FileID(1729212));
            }
        }
    }
}
