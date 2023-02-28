use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::methods::params::SourceAndTargetFile;
use crate::structures::Metadata;

pub const ENDPOINT: &str = "/renamefile";

#[derive(Serialize, Deserialize, Debug)]
pub struct RenameFile {
    pub metadata: Metadata,
}

/// Renames or moves
#[async_trait]
pub trait GetRenameFile {
    async fn renamefile(&self, input: SourceAndTargetFile) -> Result<RenameFile>;
}

#[async_trait]
impl<T: client::Client> GetRenameFile for T {
    async fn renamefile(&self, input: SourceAndTargetFile) -> Result<RenameFile> {
        let ret = self.get::<RenameFile>(ENDPOINT, input.into_params()?).await?;
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
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("renamefile.json");
        let file = fs::File::open(userinfo_json).unwrap();
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
