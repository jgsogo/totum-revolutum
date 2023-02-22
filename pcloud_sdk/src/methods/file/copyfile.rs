use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::methods::params::SourceAndTargetFile;
use crate::structures::Metadata;

pub const ENDPOINT: &str = "/copyfile";

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyFile {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetCopyFile {
    async fn copyfile(&self, input: SourceAndTargetFile) -> Result<CopyFile>;
}

#[async_trait]
impl<T: client::Client> GetCopyFile for T {
    async fn copyfile(&self, input: SourceAndTargetFile) -> Result<CopyFile> {
        let ret = self.get::<CopyFile>(ENDPOINT, input.into_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io::BufReader;
    use std::path::Path;

    use crate::types::FileID;
    use crate::utils::http::ApiResult;

    use super::*;

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
