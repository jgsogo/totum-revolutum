use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::methods::params::Params;
use crate::structures::Metadata;
use crate::types::Folder;
use crate::{client, utils};

pub const ENDPOINT: &str = "/uploadfile";

#[derive(Debug, Clone)]
pub struct UploadFileParams {
    folder: Folder,
    filename: String,
    // Optional parameters
    pub nopartial: bool,
    pub progresshash: Option<String>,
    pub renameifexists: bool,
    pub mtime: Option<i32>,
    pub ctime: Option<i32>,
}

impl UploadFileParams {
    pub fn new(folder: Folder, filename: String) -> UploadFileParams {
        UploadFileParams {
            folder,
            filename,
            nopartial: false,
            progresshash: None,
            renameifexists: false,
            mtime: None,
            ctime: None,
        }
    }
}

impl TryFrom<UploadFileParams> for HashMap<String, String> {
    type Error = anyhow::Error;

    fn try_from(value: UploadFileParams) -> std::result::Result<Self, Self::Error> {
        let mut params = HashMap::new();
        value.folder.add_to_params(&mut params)?;
        if let Some(progresshash) = value.progresshash {
            params.insert("progresshash".to_string(), progresshash);
        }
        Ok(params)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Checksum {
    sha1: String,
    sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadFile {
    pub fileids: Vec<u64>,
    pub metadata: Vec<Metadata>,
    pub checksums: Vec<Checksum>,
}

#[async_trait]
pub trait PostUploadFile: client::Client {
    async fn uploadfile(&self, local_filename: &str, upload_params: UploadFileParams) -> Result<UploadFile> {
        let filename = upload_params.filename.clone();
        let params: HashMap<String, String> = upload_params.try_into()?;
        let data = utils::http::file_data(local_filename.to_string(), &filename)?;
        // TODO: This should do some streaming (with progress bar). Probably different method to upload several files
        let ret = self.post::<UploadFile>(ENDPOINT, params, data).await?;
        Ok(ret)
    }
}

impl<T: client::Client> PostUploadFile for T {}
