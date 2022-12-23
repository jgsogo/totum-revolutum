use std::collections::HashMap;

use anyhow::{bail, Result};
use async_trait::async_trait;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::fileops::FileDescriptor;
use crate::types::{FileID, FolderID};

pub const ENDPOINT: &str = "/file_open";

bitflags! {
    pub struct Flags: u32 {
        const O_WRITE = 0x0002;
        const O_CREAT = 0x0040;
        const O_EXCL = 0x0080;
        const O_TRUNC = 0x0200;
        const O_APPEND = 0x0400;
    }
}

pub enum FileOpenPath {
    Path(String),
    FileID(FileID),
    FolderAndName(FolderID, String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileOpen {
    pub fd: FileDescriptor,
    pub fileid: FileID,
}

#[async_trait]
pub trait GetFileOpen: client::Client {
    async fn file_open(&self, flags: Flags, path: FileOpenPath) -> Result<FileOpen> {
        let mut params = HashMap::new();
        params.insert("flags".to_string(), flags.bits().to_string());

        if flags.contains(Flags::O_CREAT) {
            // When creating a file, folderid+name OR path need to be provided
            match path {
                FileOpenPath::FolderAndName(folderid, name) => {
                    params.insert("folderid".to_string(), folderid.0.to_string());
                    params.insert("name".to_string(), name);
                }
                FileOpenPath::Path(path) => {
                    params.insert("path".to_string(), path);
                }
                _ => bail!("If O_CREATE is set, provide either folderid+name or path"),
            }
        } else {
            // If the file exists, fileid or path need to be provided
            match path {
                FileOpenPath::FileID(fileid) => {
                    params.insert("fileid".to_string(), fileid.to_string());
                }
                FileOpenPath::Path(path) => {
                    params.insert("path".to_string(), path);
                }
                _ => bail!("If O_CREATE is not set, provide either fileid or path"),
            }
        }

        let ret = self.get::<FileOpen>(ENDPOINT, params).await?;
        Ok(ret)
    }
}

impl<T: client::Client> GetFileOpen for T {}
