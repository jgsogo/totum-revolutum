use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::structures::Metadata;

pub const ENDPOINT: &str = "/copyfile";

mod params {
    use crate::methods::params::ParamsType;
    use crate::methods::params::TargetFile;
    use crate::types::File;

    use super::{Params, Result};

    pub struct CopyFileInput {
        pub source: File,
        pub target: TargetFile,
    }

    impl Params for CopyFileInput {
        fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
            self.source.add_to_params(params)?;
            self.target.add_to_params(params)?;
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFile {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetCopyFile: client::Client {
    async fn copyfile(&self, input: params::CopyFileInput) -> Result<CopyFile> {
        let ret = self.get::<CopyFile>(ENDPOINT, input.into_params()?).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetCopyFile for T {}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io::BufReader;
    use std::path::{Path, PathBuf};

    use crate::methods::params::TargetFile;
    use crate::types::{File, FileID, FolderID};
    use crate::utils::http::ApiResult;

    use super::params::*;
    use super::*;

    #[test]
    fn test_params_with_ids_noname() {
        let input = CopyFileInput {
            source: File::FileID(FileID(1234)),
            target: TargetFile::FolderAndName((FolderID(4321), None)),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
    }

    #[test]
    fn test_params_with_ids_with_name() {
        let input = CopyFileInput {
            source: File::FileID(FileID(1234)),
            target: TargetFile::FolderAndName((FolderID(4321), Some("name".to_string()))),
        };
        let params = input.into_params().unwrap();
        assert_eq!(params.len(), 3);
        assert_eq!(params.get("fileid"), Some(&"1234".to_string()));
        assert_eq!(params.get("tofolderid"), Some(&"4321".to_string()));
        assert_eq!(params.get("toname"), Some(&"name".to_string()));
    }

    #[test]
    fn test_params_with_paths() {
        let input = CopyFileInput {
            source: File::Path(PathBuf::from("/from/path")),
            target: TargetFile::Path(PathBuf::from("/to/path")),
        };
        let params = input.into_params().unwrap();
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
        let file = fs::File::open(userinfo_json).unwrap();
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
