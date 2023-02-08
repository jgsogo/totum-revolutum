use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::structures::Metadata;
use crate::types::FolderID;
use crate::{client, utils};

#[derive(Debug, Clone)]
pub struct UploadFileParams {
    path: Option<String>,
    folderid: Option<FolderID>,
    filename: String,
    // Optional parameters
    pub nopartial: bool,
    pub progresshash: Option<String>,
    pub renameifexists: bool,
    pub mtime: Option<i32>,
    pub ctime: Option<i32>,
}

impl UploadFileParams {
    fn new(path: Option<String>, folderid: Option<FolderID>, filename: String) -> UploadFileParams {
        UploadFileParams {
            path,
            folderid,
            filename,
            nopartial: false,
            progresshash: None,
            renameifexists: false,
            mtime: None,
            ctime: None,
        }
    }

    pub fn new_from_path(path: String, filename: String) -> UploadFileParams {
        UploadFileParams::new(Some(path), None, filename)
    }

    pub fn new_from_folderid(folderid: FolderID, filename: String) -> UploadFileParams {
        UploadFileParams::new(None, Some(folderid), filename)
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
        let mut params = HashMap::new();
        params.insert("filename".to_string(), upload_params.filename.clone());
        // Folder-id or path, not both
        if let Some(folderid) = upload_params.folderid {
            params.insert("folderid".to_string(), folderid.to_string());
        } else if let Some(path) = upload_params.path {
            params.insert("path".to_string(), path);
        }
        if let Some(progresshash) = upload_params.progresshash {
            params.insert("progresshash".to_string(), progresshash);
        }

        let data = utils::http::file_data(local_filename.to_string(), &upload_params.filename)?;
        // TODO: This should do some streaming (with progress bar). Probably different method to upload several files
        let ret = self.post::<UploadFile>("/uploadfile", params, data).await?;
        Ok(ret)
    }
}

impl<T: client::Client> PostUploadFile for T {}
