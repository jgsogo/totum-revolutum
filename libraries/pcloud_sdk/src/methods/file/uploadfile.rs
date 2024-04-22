use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8Path;
use headers::HeaderMapExt;
use http::HeaderMap;
use mime::Mime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::params::{Params, ParamsType};
use crate::structures::MetadataFile;
use crate::types::Folder;
use crate::utils;
use crate::utils::http::BOUNDARY;

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
    async fn uploadfile(&self, local_filename: &Utf8Path, upload_params: UploadFileParams) -> Result<UploadFile>;
}

#[async_trait]
impl<T: PCloudClient> PostUploadFile for T {
    async fn uploadfile(&self, local_filename: &Utf8Path, upload_params: UploadFileParams) -> Result<UploadFile> {
        let mut headers = HeaderMap::default();
        let mime_multipart = Mime::from_str(&format!("multipart/form-data; boundary={BOUNDARY}")).unwrap();
        let content_type = headers::ContentType::from(mime_multipart);
        headers.typed_insert(content_type);

        let filename = upload_params.filename.clone();
        let data = utils::http::create_file_data(local_filename, &filename)?;
        let ret = RESTClient::post::<UploadFile>(self, ENDPOINT, headers, upload_params.into_params()?, data).await?;
        Ok(ret)
    }
}
