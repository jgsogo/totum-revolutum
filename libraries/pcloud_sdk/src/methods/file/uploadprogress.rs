use std::collections::HashMap;

use crate::client::PCloudClient;
use crate::structures::MetadataFile;
use crate::Result;
use async_trait::async_trait;
use http::HeaderMap;
use http_utils::rest::RESTClient;
use serde::{Deserialize, Serialize};

pub const ENDPOINT: &str = "/uploadprogress";

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadProgressData {
    pub total: u64,
    pub uploaded: u64,
    pub currentfile: Option<String>,
    pub files: Vec<MetadataFile>,
    pub finished: bool,
}

#[async_trait]
pub trait UploadProgress {
    async fn uploadprogress(&self, progresshash: &str) -> Result<UploadProgressData>;
}

#[async_trait]
impl<T: PCloudClient> UploadProgress for T
where
    T: http_utils::HttpClient<Error = crate::Error>,
{
    async fn uploadprogress(&self, progresshash: &str) -> Result<UploadProgressData> {
        let mut params = HashMap::new();
        params.insert("progresshash".to_string(), progresshash.to_string());
        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &params).await
    }
}
