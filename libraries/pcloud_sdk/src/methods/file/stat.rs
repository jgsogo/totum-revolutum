use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::params::Params;
use crate::structures::MetadataFile;
use crate::types::File;

pub const ENDPOINT: &str = "/stat";

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub metadata: MetadataFile,
}

#[async_trait]
pub trait GetStat {
    async fn stat(&self, input: File) -> Result<Stat>;
}

#[async_trait]
impl<T: PCloudClient> GetStat for T {
    async fn stat(&self, input: File) -> Result<Stat> {
        let ret = RESTClient::get::<Stat>(self, ENDPOINT, HeaderMap::default(), input.into_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::io::BufReader;

    use camino::Utf8Path;

    use crate::types::FileID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("stat.json");
        let file = fs::File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<Stat>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.fileid, FileID(1729212));
            }
        }
    }
}
