use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::client;
use crate::types::File;

pub struct GetFileLinkInput {
    // `FileID` or path (`String`) to the file
    file: File,

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
    pub fn new(file: File) -> GetFileLinkInput {
        GetFileLinkInput {
            file,
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
        let mut params = HashMap::new();
        match file_link {
            GetFileLinkInput {
                file: File::Path(p), ..
            } => {
                params.insert("path".to_string(), p.to_string_lossy().to_string());
            }
            GetFileLinkInput {
                file: File::FileID(f), ..
            } => {
                params.insert("fileid".to_string(), f.0.to_string());
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::mocks::client::MockLocalClient;
    use crate::types::FileID;

    use super::*;

    #[test]
    fn test_getfilelinkinput_defaults() {
        let path = File::from_str("/path/to/file").unwrap();
        let input = GetFileLinkInput::new(path.clone());
        assert_eq!(input.file, path);
        assert_eq!(input.forcedownload, false);
        assert_eq!(input.contenttype, None);
        assert_eq!(input.maxspeed, None);
        assert_eq!(input.skipfilename, false);
    }

    #[tokio::test]
    async fn test_getfilelink_from_path() -> Result<()> {
        let mut input = GetFileLinkInput::new(File::from_str("/the/path").unwrap());
        input.skipfilename = true;
        input.contenttype = Some("<contenttype>".to_string());
        input.maxspeed = Some(200);
        input.forcedownload = true;

        let utc_now = OffsetDateTime::now_utc();
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/getfilelink");
                assert_eq!(params.len(), 5);
                assert_eq!(params.get("fileid"), None);
                assert_eq!(params.get("path"), Some(&"/the/path".to_string()));
                assert_eq!(params.get("forcedownload"), Some(&"1".to_string()));
                assert_eq!(params.get("contenttype"), Some(&"<contenttype>".to_string()));
                assert_eq!(params.get("maxspeed"), Some(&"200".to_string()));
                assert_eq!(params.get("skipfilename"), Some(&"1".to_string()));

                Ok(FileLink {
                    result: 0,
                    path: "<path>".to_string(),
                    expires: utc_now.clone(),
                    hosts: vec!["host1".to_string(), "host2".to_string()],
                })
            });

        let filelink = client.getfilelink(&input).await?;
        assert_eq!(filelink.result, 0);
        assert_eq!(filelink.path, "<path>".to_string());
        assert_eq!(filelink.expires, utc_now);
        assert_eq!(filelink.hosts, vec!["host1".to_string(), "host2".to_string()]);

        Ok(())
    }

    #[tokio::test]
    async fn test_getfilelink_from_fileid() -> Result<()> {
        let input = GetFileLinkInput::new(File::FileID(FileID(42)));

        let utc_now = OffsetDateTime::now_utc();
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/getfilelink");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fileid"), Some(&"42".to_string()));
                assert_eq!(params.get("path"), None);

                Ok(FileLink {
                    result: 0,
                    path: "<path>".to_string(),
                    expires: utc_now.clone(),
                    hosts: vec!["host1".to_string(), "host2".to_string()],
                })
            });

        let filelink = client.getfilelink(&input).await?;
        assert_eq!(filelink.result, 0);
        assert_eq!(filelink.path, "<path>".to_string());
        assert_eq!(filelink.expires, utc_now);
        assert_eq!(filelink.hosts, vec!["host1".to_string(), "host2".to_string()]);

        Ok(())
    }
}
