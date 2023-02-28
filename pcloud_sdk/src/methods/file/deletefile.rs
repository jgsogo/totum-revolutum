use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::structures::Metadata;
use crate::types::File;

pub const ENDPOINT: &str = "/deletefile";

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFile {
    pub id: String,
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetDeleteFile {
    async fn deletefile(&self, input: File) -> Result<DeleteFile>;
}

#[async_trait]
impl<T: client::Client> GetDeleteFile for T {
    async fn deletefile(&self, input: File) -> Result<DeleteFile> {
        let ret = self.get::<DeleteFile>(ENDPOINT, input.into_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use camino::Utf8Path;
    use std::env;
    use std::fs;
    use std::io::BufReader;

    use crate::types::FileID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("deletefile.json");
        let file = fs::File::open(userinfo_json).unwrap();
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
