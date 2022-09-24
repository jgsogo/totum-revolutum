use std::collections::HashMap;

use async_trait::async_trait;
use hyper::client::connect::Connect;
use serde::{Deserialize, Serialize};

use crate::{client, utils};
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
pub trait UploadProgress<C>: client::Client<C>
    where
        C: Connect + Clone + Send + Sync + 'static
{
    async fn uploadprogress(&self, progresshash: &str) -> Result<UploadProgressData, hyper::Error>
        where
            C: Connect + Clone + Send + Sync + 'static
    {
        let url = format!("https://{}/uploadprogress", self.hostname());
        let mut params = HashMap::new();
        params.insert("progresshash".to_string(), progresshash.to_string());
        let ret = self.get::<UploadProgressData>(&url, params).await?;
        Ok(ret)
    }
}

impl<C, T: client::Client<C>> UploadProgress<C> for T
    where
        C: Connect + Clone + Send + Sync + 'static {}
