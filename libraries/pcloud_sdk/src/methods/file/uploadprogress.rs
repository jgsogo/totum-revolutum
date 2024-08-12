use std::collections::HashMap;

use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use utils::http::rest::RESTClient;

use crate::client::PCloudClient;
use crate::structures::MetadataFile;
use crate::Result;

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
impl<T: PCloudClient> UploadProgress for T {
    async fn uploadprogress(&self, progresshash: &str) -> Result<UploadProgressData> {
        let mut params = HashMap::new();
        params.insert("progresshash".to_string(), progresshash.to_string());
        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &params).await
    }
}
