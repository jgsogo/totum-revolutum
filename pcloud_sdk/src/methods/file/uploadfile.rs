use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::methods::params::{Params, ParamsType};
use crate::structures::MetadataFile;
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

impl Params for UploadFileParams {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        self.folder.add_to_params(params)?;
        if let Some(progresshash) = &self.progresshash {
            params.insert("progresshash".to_string(), progresshash.clone());
        }
        Ok(())
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
    pub metadata: Vec<MetadataFile>,
    pub checksums: Vec<Checksum>,
}

#[async_trait]
pub trait PostUploadFile {
    async fn uploadfile(&self, local_filename: &str, upload_params: UploadFileParams) -> Result<UploadFile>;
}

#[async_trait]
impl<T: client::Client> PostUploadFile for T {
    async fn uploadfile(&self, local_filename: &str, upload_params: UploadFileParams) -> Result<UploadFile> {
        let filename = upload_params.filename.clone();
        let data = utils::http::file_data(local_filename.to_string(), &filename)?;
        let ret = self
            .post::<UploadFile>(ENDPOINT, upload_params.into_params()?, data)
            .await?;
        Ok(ret)
    }
}
