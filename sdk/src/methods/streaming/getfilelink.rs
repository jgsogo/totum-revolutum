use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::client;
use crate::id::FileID;

enum GetFileLinkFileInput {
    FileID(FileID),
    Path(String),
}

pub struct GetFileLinkInput {
    // `FileID` or path (`String`) to the file
    file: GetFileLinkFileInput,

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
    pub fn new_from_path(path: &str) -> GetFileLinkInput {
        GetFileLinkInput {
            file: GetFileLinkFileInput::Path(path.to_string()),
            forcedownload: false,
            contenttype: None,
            maxspeed: None,
            skipfilename: false,
        }
    }
    pub fn new_from_fileid(file: &FileID) -> GetFileLinkInput {
        GetFileLinkInput {
            file: GetFileLinkFileInput::FileID(file.clone()),
            forcedownload: false,
            contenttype: None,
            maxspeed: None,
            skipfilename: false,
        }
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
        let url = format!("https://{}/getfilelink", self.hostname());
        let mut params = HashMap::new();
        match file_link {
            GetFileLinkInput {
                file: GetFileLinkFileInput::Path(p),
                ..
            } => {
                params.insert("path".to_string(), p.clone());
            }
            GetFileLinkInput {
                file: GetFileLinkFileInput::FileID(f),
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

        let ret = self.get::<FileLink>(&url, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetFileLink for T {}
