use std::time::Duration;

use async_trait::async_trait;
use headers::HeaderMapExt;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use utils::http::rest::RESTClient;

use crate::client::PCloudClient;
use crate::structures::MetadataFile;
use crate::types::File;
use crate::Result;

pub const ENDPOINT: &str = "/stat";

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub metadata: MetadataFile,
}

#[async_trait]
pub trait GetStat {
    /// Calls the `stat` endpoint. It returns information about a file.
    async fn stat(&self, input: &File) -> Result<Stat>;

    /// Calls the `stat` endpoint. Use `retry_condition` to decide if the method should be
    /// called again or not (return Err or Ok), this can be useful when some optional data is not
    /// available yet, and we want to give the server a bit more time to compute it (like
    /// `metadata.hash` and `metadata.size`.
    async fn stat_with_retry(
        &self,
        input: &File,
        retry_condition: Box<dyn Fn(Result<Stat>) -> Result<Stat> + Send + Sync>,
    ) -> Result<Stat>;
}

#[async_trait]
impl<T: PCloudClient> GetStat for T {
    async fn stat(&self, input: &File) -> Result<Stat> {
        let mut headers = HeaderMap::default();
        let conn = headers::Connection::close();
        headers.typed_insert(conn);

        RESTClient::get(self, ENDPOINT, headers, input).await
    }

    async fn stat_with_retry(
        &self,
        input: &File,
        retry_condition: Box<dyn Fn(Result<Stat>) -> Result<Stat> + Send + Sync>,
    ) -> Result<Stat> {
        tryhard::retry_fn(|| stat_with_retry(self, input, &retry_condition))
            .retries(10)
            .exponential_backoff(Duration::from_millis(100))
            .max_delay(Duration::from_secs(5))
            .await
    }
}

async fn stat_with_retry<T: PCloudClient>(
    client: &T,
    input: &File,
    retry_condition: &(dyn Fn(Result<Stat>) -> Result<Stat> + Send + Sync),
) -> Result<Stat> {
    let r = client.stat(input).await;
    retry_condition(r)
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
                assert_eq!(data.metadata.fileid, FileID::new(1729212));
            }
        }
    }
}
