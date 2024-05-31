use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8Path;
use headers::HeaderMapExt;
use http::HeaderMap;
use http_utils::AddToParams;
use mime::Mime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::structures::MetadataFile;
use crate::types::FolderID;
use crate::utils;
use crate::utils::http::BOUNDARY;

pub const ENDPOINT: &str = "/uploadfile";

#[derive(Debug, Clone)]
pub struct UploadFileParams {
    folderid: FolderID,
    filename: String,
    // Optional parameters
    pub nopartial: bool,
    pub progresshash: Option<String>,
    pub renameifexists: bool,
    pub mtime: Option<i32>,
    pub ctime: Option<i32>,
}

impl UploadFileParams {
    pub fn new(folderid: FolderID, filename: String) -> UploadFileParams {
        UploadFileParams {
            folderid,
            filename,
            nopartial: false,
            progresshash: None,
            renameifexists: false,
            mtime: None,
            ctime: None,
        }
    }
}

impl AddToParams for UploadFileParams {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        self.folderid.add_to_params(params);
        if let Some(progresshash) = &self.progresshash {
            params.insert("progresshash".to_string(), progresshash.clone());
        }
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
    /// Uploads a local file to the given [`FolderID`]
    ///
    /// Only `folderid`+`name` alternative is implemented as it's the one recommended in the
    /// documentation.
    ///
    /// Link: <https://docs.pcloud.com/methods/file/uploadfile.html>
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
        RESTClient::post(self, ENDPOINT, headers, &upload_params, data).await
    }
}
