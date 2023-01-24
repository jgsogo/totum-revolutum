use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::FileID;

pub const ENDPOINT: &str = "/deletefile";

pub enum DeleteFileInput {
    FileID(FileID),
    Path(PathBuf),
}

impl TryFrom<DeleteFileInput> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: DeleteFileInput) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        match value {
            DeleteFileInput::FileID(fid) => {
                params.insert("fileid".to_string(), fid.0.to_string());
            }
            DeleteFileInput::Path(p) => {
                params.insert("path".to_string(), p.to_string_lossy().parse()?);
            }
        };
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFile {
    pub id: String,
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetDeletefile: client::Client {
    async fn deletefile(&self, input: DeleteFileInput) -> Result<DeleteFile> {
        let params = HashMap::try_from(input)?;
        let ret = self.get::<DeleteFile>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetDeletefile for T {}

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
        let input = DeleteFileInput::FileID(FileID(1234));
        let params: HashMap<String, String> = HashMap::try_from(input).unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
    }

    #[test]
    fn test_params_with_path() {
        let input = DeleteFileInput::Path(PathBuf::from("/this/is/the/path"));
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
            .join("deletefile.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<DeleteFile>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.fileid.unwrap(), FileID(1736716));
                assert_eq!(data.id, "139-0".to_string());
            }
        }
    }
}
