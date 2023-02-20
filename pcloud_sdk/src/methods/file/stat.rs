use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::Params;
use crate::structures::Metadata;
use crate::types::File;

pub const ENDPOINT: &str = "/stat";

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetStat: client::Client {
    async fn stat(&self, input: File) -> Result<Stat> {
        let mut params = HashMap::new();
        input.add_to_params(&mut params)?;
        let ret = self.get::<Stat>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetStat for T {}

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
    fn test_deserialize() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
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
                assert_eq!(data.metadata.fileid.unwrap(), FileID(1729212));
            }
        }
    }
}
