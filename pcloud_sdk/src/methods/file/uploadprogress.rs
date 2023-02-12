use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadProgressData {
    pub total: u64,
    pub uploaded: u64,
    pub currentfile: Option<String>,
    pub files: Vec<Metadata>,
    pub finished: bool,
}

#[async_trait]
pub trait UploadProgress: client::Client {
    async fn uploadprogress(&self, progresshash: &str) -> Result<UploadProgressData> {
        let mut params = HashMap::new();
        params.insert("progresshash".to_string(), progresshash.to_string());
        let ret = self.get::<UploadProgressData>("/uploadprogress", params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> UploadProgress for T {}
