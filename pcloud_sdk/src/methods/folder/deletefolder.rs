use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::structures::Metadata;
use crate::types::Folder;

pub const ENDPOINT: &str = "/deletefolder";

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteFolder {
    pub id: String,
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetDeleteFolder {
    async fn deletefolder(&self, input: Folder) -> Result<DeleteFolder>;
}

#[async_trait]
impl<T: client::Client> GetDeleteFolder for T {
    async fn deletefolder(&self, input: Folder) -> Result<DeleteFolder> {
        let ret = self.get::<DeleteFolder>(ENDPOINT, input.into_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::types::FolderID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("deletefolder.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<DeleteFolder>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.folderid.unwrap(), FolderID(230807));
                assert_eq!(data.id, "111-0".to_string());
            }
        }
    }
}
