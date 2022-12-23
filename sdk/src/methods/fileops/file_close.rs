use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::fileops::FileDescriptor;

pub const ENDPOINT: &str = "/file_close";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileClose {
    pub bytes: Vec<u8>,
}

#[async_trait]
pub trait GetFileClose: client::Client {
    async fn file_close(&self, descriptor: FileDescriptor) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("fd".to_string(), descriptor.to_string());
        self.get::<()>(ENDPOINT, params).await
    }
}

impl<T: client::Client> GetFileClose for T {}
