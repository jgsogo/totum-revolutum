use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::client;
use crate::types::FileID;
use crate::types::PCloudFile;

pub struct GetFileLinkInput {
    // `FileID` or path (`String`) to the file
    file: PCloudFile,

    // Download with Content-Type = application/octet-stream
    pub forcedownload: bool,

    // Set Content-Type
    pub contenttype: Option<String>,

    // limit the download speed
    pub maxspeed: Option<i32>,

    // Include the name of the file in the generated link
    pub skipfilename: bool,
}

impl GetFileLinkInput {
    pub fn new_from_file(file: PCloudFile) -> GetFileLinkInput {
        GetFileLinkInput {
            file: file,
            forcedownload: false,
            contenttype: None,
            maxspeed: None,
            skipfilename: false,
        }
    }

    pub fn new_from_path(path: &str) -> GetFileLinkInput {
        Self::new_from_file(PCloudFile::Path(path.to_string()))
    }
    pub fn new_from_fileid(file: &FileID) -> GetFileLinkInput {
        Self::new_from_file(PCloudFile::FileID(file.clone()))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLink {
    pub result: u16,
    pub(crate) path: String,
    #[serde(with = "time::serde::rfc2822")]
    expires: OffsetDateTime,
    pub(crate) hosts: Vec<String>,
}

#[async_trait]
pub trait GetFileLink: client::Client {
    async fn getfilelink(&self, file_link: &GetFileLinkInput) -> Result<FileLink> {
        let mut params = HashMap::new();
        match file_link {
            GetFileLinkInput {
                file: PCloudFile::Path(p),
                ..
            } => {
                params.insert("path".to_string(), p.clone());
            }
            GetFileLinkInput {
                file: PCloudFile::FileID(f),
                ..
            } => {
                params.insert("fileid".to_string(), f.id().to_string());
            }
        }

        if file_link.forcedownload {
            params.insert("forcedownload".to_string(), "1".to_string());
        }

        if let Some(ct) = &file_link.contenttype {
            params.insert("contenttype".to_string(), ct.to_string());
        }

        if let Some(v) = &file_link.maxspeed {
            params.insert("maxspeed".to_string(), v.to_string());
        }

        if file_link.skipfilename {
            params.insert("skipfilename".to_string(), "1".to_string());
        }

        let ret = self.get::<FileLink>("/getfilelink", params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetFileLink for T {}
