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
pub trait GetFileOpen {
    async fn file_open(&self, flags: Flags, path: FileOpenPath) -> Result<FileOpen>;
}

#[async_trait]
impl<T: client::Client> GetFileOpen for T {
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
                    params.insert("fileid".to_string(), fileid.0.to_string());
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

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_ocreate() -> Result<()> {
        // test we get the expected error
        {
            let flags = Flags::O_CREAT;
            let client = MockLocalClient::new();
            let r = client.file_open(flags, FileOpenPath::FileID(FileID(42))).await;
            assert!(r.is_err());
            assert_eq!(
                r.unwrap_err().to_string(),
                "If O_CREATE is set, provide either folderid+name or path"
            );
        }

        // test O_CREAT with folderid + name
        {
            let flags = Flags::O_CREAT;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, params: HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 3);
                    assert_eq!(params.get("flags"), Some(&"64".to_string()));
                    assert_eq!(params.get("folderid"), Some(&"42".to_string()));
                    assert_eq!(params.get("name"), Some(&"name".to_string()));
                    Ok(FileOpen {
                        fd: 42,
                        fileid: FileID(42),
                    })
                });
            let input = FileOpenPath::FolderAndName(FolderID(42), "name".to_string());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, 42);
            assert_eq!(r.fileid, FileID(42));
        }

        // test O_CREAT with path
        {
            let flags = Flags::O_CREAT | Flags::O_APPEND;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, params: HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"1088".to_string()));
                    assert_eq!(params.get("path"), Some(&"the/path".to_string()));
                    Ok(FileOpen {
                        fd: 42,
                        fileid: FileID(42),
                    })
                });
            let input = FileOpenPath::Path("the/path".to_string());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, 42);
            assert_eq!(r.fileid, FileID(42));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_no_ocreate() -> Result<()> {
        // test we get the expected error
        {
            let flags = Flags::empty();
            let client = MockLocalClient::new();
            let r = client
                .file_open(flags, FileOpenPath::FolderAndName(FolderID(42), "name".to_string()))
                .await;
            assert!(r.is_err());
            assert_eq!(
                r.unwrap_err().to_string(),
                "If O_CREATE is not set, provide either fileid or path"
            );
        }

        // test ~O_CREAT with fileid
        {
            let flags = Flags::empty();
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, params: HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"0".to_string()));
                    assert_eq!(params.get("fileid"), Some(&"42".to_string()));
                    Ok(FileOpen {
                        fd: 42,
                        fileid: FileID(42),
                    })
                });
            let input = FileOpenPath::FileID(FileID(42));
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, 42);
            assert_eq!(r.fileid, FileID(42));
        }

        // test ~O_CREAT with path
        {
            let flags = Flags::empty();
            let flags = flags | Flags::O_EXCL;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, params: HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"128".to_string()));
                    assert_eq!(params.get("path"), Some(&"the/path".to_string()));
                    Ok(FileOpen {
                        fd: 42,
                        fileid: FileID(42),
                    })
                });
            let input = FileOpenPath::Path("the/path".to_string());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, 42);
            assert_eq!(r.fileid, FileID(42));
        }

        Ok(())
    }
}
