use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::{FileID, FolderID};

pub const ENDPOINT: &str = "/copyfile";

pub enum CopyFileSourceInput {
    FileID(FileID),
    Path(PathBuf),
}

pub struct CopyFileTargetID {
    folderid: FolderID,
    name: Option<String>,
}

pub enum CopyFileTargetInput {
    FolderID(CopyFileTargetID),
    Path(PathBuf),
}

pub struct CopyFileInput {
    source: CopyFileSourceInput,
    target: CopyFileTargetInput,
}

impl TryFrom<CopyFileInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: CopyFileInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        let CopyFileInput { source, target } = value;
        match source {
            CopyFileSourceInput::FileID(fid) => {
                params.insert("fileid".to_string(), fid.0.to_string());
            }
            CopyFileSourceInput::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        match target {
            CopyFileTargetInput::FolderID(target) => {
                params.insert("tofolderid".to_string(), target.folderid.0.to_string());
                if let Some(name) = target.name {
                    params.insert("toname".to_string(), name);
                }
            }
            CopyFileTargetInput::Path(p) => {
                params.insert("topath".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFile {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetCopyFile: client::Client {
    async fn copyfile(&self, input: CopyFileInput) -> Result<CopyFile> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<CopyFile>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetCopyFile for T {}

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
        let input = CopyFileInput {
            source: CopyFileSourceInput::FileID(FileID(1234)),
            target: CopyFileTargetInput::FolderID(CopyFileTargetID {
                folderid: FolderID(4321),
                name: None,
            }),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = CopyFileInput {
            source: CopyFileSourceInput::FileID(FileID(1234)),
            target: CopyFileTargetInput::FolderID(CopyFileTargetID {
                folderid: FolderID(4321),
                name: Some("name".to_string()),
            }),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = CopyFileInput {
            source: CopyFileSourceInput::Path(PathBuf::from("/from/path")),
            target: CopyFileTargetInput::Path(PathBuf::from("/to/path")),
        };
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("path"), Some(&"/from/path".to_string()));
        assert_eq!(params.get("topath"), Some(&"/to/path".to_string()));
    }

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("copyfile.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<CopyFile>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.fileid.unwrap(), FileID(1732283));
            }
        }
    }
}
