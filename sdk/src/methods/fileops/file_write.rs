use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::fileops::FileDescriptor;

pub const ENDPOINT: &str = "/file_write";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileWrite {
    pub bytes: u64,
}

#[async_trait]
pub trait PostFileWrite: client::Client {
    async fn file_write(&self, descriptor: FileDescriptor, data: Vec<u8>) -> Result<FileWrite> {
        let mut params = HashMap::new();
        params.insert("fd".to_string(), descriptor.to_string());

        let ret = self.post::<FileWrite>(ENDPOINT, params, data).await?;
        Ok(ret)
    }
}

impl<T: client::Client> PostFileWrite for T {}
