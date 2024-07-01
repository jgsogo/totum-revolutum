use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use headers::HeaderMapExt;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use utils::http::AddToParams;

use utils::http::rest::RESTClient;

use crate::client::PCloudClient;
use crate::structures::MetadataFile;
use crate::types::File;
use crate::Result;

pub const ENDPOINT: &str = "/checksumfile";

#[derive(Serialize, Deserialize, Debug)]
pub struct ChecksumFile {
    pub sha1: Option<String>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub metadata: MetadataFile,
}

#[async_trait]
pub trait GetChecksumFile {
    async fn checksumfile(&self, input: &File) -> Result<ChecksumFile>;

    /// Calls the `checksumfile` endpoint. Use `retry_condition` to decide if the method should be
    /// called again or not (return Err or Ok), this can be useful when some optional data is not
    /// available yet, and we want to give the server a bit more time to compute it (like
    /// `metadata.hash` and `metadata.size`).
    async fn checksumfile_with_retry(
        &self,
        input: &File,
        retry_condition: Box<dyn Fn(Result<ChecksumFile>) -> Result<ChecksumFile> + Send + Sync>,
    ) -> Result<ChecksumFile>;
}

#[async_trait]
impl<T: PCloudClient> GetChecksumFile for T {
    async fn checksumfile(&self, input: &File) -> Result<ChecksumFile> {
        // Closing the connection here (expected to work at least on the second call). When I've
        // just uploaded a file, some optional information (hash) is not available right away. We
        // need to retry (or maybe just use a new connection)
        let mut headers = HeaderMap::default();
        let conn = headers::Connection::close();
        headers.typed_insert(conn);

        let mut params = HashMap::new();
        input.add_to_params(&mut params);

        RESTClient::get(self, ENDPOINT, headers, &params).await
    }

    async fn checksumfile_with_retry(
        &self,
        input: &File,
        retry_condition: Box<dyn Fn(Result<ChecksumFile>) -> Result<ChecksumFile> + Send + Sync>,
    ) -> Result<ChecksumFile> {
        tryhard::retry_fn(|| checksumfile_with_retry(self, input, &retry_condition))
            .retries(10)
            .exponential_backoff(Duration::from_millis(100))
            .max_delay(Duration::from_secs(5))
            .await
    }
}

async fn checksumfile_with_retry<T: PCloudClient>(
    client: &T,
    input: &File,
    retry_condition: &(dyn Fn(Result<ChecksumFile>) -> Result<ChecksumFile> + Send + Sync),
) -> Result<ChecksumFile> {
    let r = client.checksumfile(input).await;
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
            .join("checksumfile.json");
        let file = fs::File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<ChecksumFile>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.sha1, Some("e96d1afe9846905e63f52a7c882ee7f7c889ecc7".to_string()));
                assert_eq!(
                    data.sha256,
                    Some("d65f0ea8df306d5ada3f25ee771f88828ba75bd58ee3f87b83e67fc07abfd745".to_string())
                );
                assert_eq!(data.md5, None);
                assert_eq!(data.metadata.fileid, FileID::new(45574261475));
            }
        }
    }
}
