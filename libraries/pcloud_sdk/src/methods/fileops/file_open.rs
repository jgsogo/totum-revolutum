use std::collections::HashMap;

use async_trait::async_trait;
use bitflags::bitflags;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use utils::http::rest::RESTClient;
use utils::http::AddToParams;

use crate::client::PCloudClient;
use crate::methods::fileops::FileDescriptor;
use crate::types::{File, FileID, FolderID};
use crate::{Error, Result};

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
    File(File),
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
impl<T: PCloudClient> GetFileOpen for T {
    async fn file_open(&self, flags: Flags, path: FileOpenPath) -> Result<FileOpen> {
        let mut params = HashMap::new();
        params.insert("flags".to_string(), flags.bits().to_string());

        if flags.contains(Flags::O_CREAT) {
            // When creating a file, folderid+name OR path need to be provided
            match path {
                FileOpenPath::FolderAndName(folderid, name) => {
                    folderid.add_to_params(&mut params);
                    params.insert("name".to_string(), name);
                }
                FileOpenPath::File(File::RemotePath(path)) => {
                    path.add_to_params(&mut params);
                }
                _ => {
                    return Err(Error::InputDataError(
                        "If O_CREATE is set, provide either folderid+name or path".to_string(),
                    ))
                }
            }
        } else {
            // If the file exists, fileid or path need to be provided
            match path {
                FileOpenPath::File(file) => file.add_to_params(&mut params),
                _ => {
                    return Err(Error::InputDataError(
                        "If O_CREATE is not set, provide either fileid or path".to_string(),
                    ))
                }
            }
        }

        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &params).await
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_ocreate() -> Result<()> {
        // test we get the expected error
        {
            let flags = Flags::O_CREAT;
            let client = MockLocalClient::new();
            let r = client
                .file_open(flags, FileOpenPath::File(FileID::new(42).into()))
                .await;
            assert!(r.is_err());
            assert!(
                matches!(r.unwrap_err(), Error::InputDataError(ref message) if message == "If O_CREATE is set, provide either folderid+name or path")
            );
        }

        // test O_CREAT with folderid + name
        {
            let flags = Flags::O_CREAT;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 3);
                    assert_eq!(params.get("flags"), Some(&"64".to_string()));
                    assert_eq!(params.get("folderid"), Some(&"42".to_string()));
                    assert_eq!(params.get("name"), Some(&"name".to_string()));
                    assert_eq!(headers.len(), 0);
                    Ok(FileOpen {
                        fd: FileDescriptor::new(42),
                        fileid: FileID::new(42),
                    })
                });
            let input = FileOpenPath::FolderAndName(FolderID::new(42), "name".to_string());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, FileDescriptor::new(42));
            assert_eq!(r.fileid, FileID::new(42));
        }

        // test O_CREAT with path
        {
            let flags = Flags::O_CREAT | Flags::O_APPEND;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"1088".to_string()));
                    assert_eq!(params.get("path"), Some(&"/the/path".to_string()));
                    assert_eq!(headers.len(), 0);
                    Ok(FileOpen {
                        fd: FileDescriptor::new(42),
                        fileid: FileID::new(42),
                    })
                });
            let input = FileOpenPath::File(File::from_str("path:/the/path").unwrap());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, FileDescriptor::new(42));
            assert_eq!(r.fileid, FileID::new(42));
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
                .file_open(
                    flags,
                    FileOpenPath::FolderAndName(FolderID::new(42), "name".to_string()),
                )
                .await;
            assert!(r.is_err());
            assert!(
                matches!(r.unwrap_err(), Error::InputDataError(ref message) if message == "If O_CREATE is not set, provide either fileid or path")
            );
        }

        // test ~O_CREAT with fileid
        {
            let flags = Flags::empty();
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"0".to_string()));
                    assert_eq!(params.get("fileid"), Some(&"42".to_string()));
                    assert_eq!(headers.len(), 0);
                    Ok(FileOpen {
                        fd: FileDescriptor::new(42),
                        fileid: FileID::new(42),
                    })
                });
            let input = FileOpenPath::File(FileID::new(42).into());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, FileDescriptor::new(42));
            assert_eq!(r.fileid, FileID::new(42));
        }

        // test ~O_CREAT with path
        {
            let flags = Flags::empty();
            let flags = flags | Flags::O_EXCL;
            let mut client = MockLocalClient::new();
            client
                .expect_get()
                .times(1)
                .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                    assert_eq!(endpoint, "/file_open");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params.get("flags"), Some(&"128".to_string()));
                    assert_eq!(params.get("path"), Some(&"/the/path".to_string()));
                    assert_eq!(headers.len(), 0);
                    Ok(FileOpen {
                        fd: FileDescriptor::new(42),
                        fileid: FileID::new(42),
                    })
                });
            let input = FileOpenPath::File(File::from_str("path:/the/path").unwrap());
            let r = client.file_open(flags, input).await?;
            assert_eq!(r.fd, FileDescriptor::new(42));
            assert_eq!(r.fileid, FileID::new(42));
        }

        Ok(())
    }
}
