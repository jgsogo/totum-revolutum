use anyhow::Result;
use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::debug;

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::params::{Params, ParamsType};
use crate::types::File;

pub const ENDPOINT: &str = "/getfilelink";

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

impl Params for GetFileLinkInput {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        self.file.add_to_params(params)?;
        if self.forcedownload {
            params.insert("forcedownload".to_string(), "1".to_string());
        }
        if let Some(ct) = &self.contenttype {
            params.insert("contenttype".to_string(), ct.to_string());
        }
        if let Some(v) = &self.maxspeed {
            params.insert("maxspeed".to_string(), v.to_string());
        }
        if self.skipfilename {
            params.insert("skipfilename".to_string(), "1".to_string());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileLink {
    pub(crate) path: String,
    #[serde(with = "time::serde::rfc2822")]
    expires: OffsetDateTime,
    pub(crate) hosts: Vec<String>,
}

#[async_trait]
pub trait GetFileLink {
    async fn getfilelink(&self, file_link: GetFileLinkInput) -> Result<FileLink>;
}

#[async_trait]
impl<T: PCloudClient> GetFileLink for T {
    async fn getfilelink(&self, file_link: GetFileLinkInput) -> Result<FileLink> {
        debug!("pcloud::getfilelink - file '{}'", file_link.file);
        let ret = RESTClient::get::<FileLink>(self, ENDPOINT, HeaderMap::default(), file_link.create_params()?).await?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::fs::File as FsFile;
    use std::io::BufReader;
    use std::str::FromStr;

    use camino::Utf8Path;
    use time::macros::datetime;

    use crate::mocks::client::MockLocalClient;
    use crate::types::FileID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_getfilelink() {
        fn reader() -> BufReader<FsFile> {
            let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
            let getfilelink_json = Utf8Path::new(&manifest_dir)
                .join("resources")
                .join("testdata")
                .join("getfilelink.json");
            let file = FsFile::open(getfilelink_json).unwrap();
            BufReader::new(file)
        }

        // Deserialize using ApiResult wrapper
        match serde_json::from_reader::<_, ApiResult<FileLink>>(reader()) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                assert_eq!(data.error, None);
                let filelink = data.data.unwrap();
                assert_eq!(filelink.path, "/lkjasdffdai99009asfda/name2.txt".to_string());
                assert_eq!(filelink.expires, datetime!(2023-03-14 22:06:18 UTC));
                assert_eq!(filelink.hosts, vec!["evc23.pcloud.com", "evc300.pcloud.com"]);
            }
        }
    }

    #[test]
    fn test_getfilelinkinput_defaults() {
        let path = File::from_str("path:/path/to/file").unwrap();
        let input = GetFileLinkInput::new(path.clone());
        assert_eq!(input.file, path);
        assert_eq!(input.forcedownload, false);
        assert_eq!(input.contenttype, None);
        assert_eq!(input.maxspeed, None);
        assert_eq!(input.skipfilename, false);
    }

    #[tokio::test]
    async fn test_getfilelink_from_path() -> Result<()> {
        let mut input = GetFileLinkInput::new(File::from_str("path:/the/path").unwrap());
        input.skipfilename = true;
        input.contenttype = Some("<contenttype>".to_string());
        input.maxspeed = Some(200);
        input.forcedownload = true;

        let utc_now = OffsetDateTime::now_utc();
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/getfilelink");
                assert_eq!(params.len(), 5);
                assert_eq!(params.get("fileid"), None);
                assert_eq!(params.get("path"), Some(&"/the/path".to_string()));
                assert_eq!(params.get("forcedownload"), Some(&"1".to_string()));
                assert_eq!(params.get("contenttype"), Some(&"<contenttype>".to_string()));
                assert_eq!(params.get("maxspeed"), Some(&"200".to_string()));
                assert_eq!(params.get("skipfilename"), Some(&"1".to_string()));
                assert_eq!(headers.len(), 0);

                Ok(FileLink {
                    path: "<path>".to_string(),
                    expires: utc_now.clone(),
                    hosts: vec!["host1".to_string(), "host2".to_string()],
                })
            });

        let filelink = client.getfilelink(input).await?;
        assert_eq!(filelink.path, "<path>".to_string());
        assert_eq!(filelink.expires, utc_now);
        assert_eq!(filelink.hosts, vec!["host1".to_string(), "host2".to_string()]);

        Ok(())
    }

    #[tokio::test]
    async fn test_getfilelink_from_fileid() -> Result<()> {
        let input = GetFileLinkInput::new(FileID::new(42).into());

        let utc_now = OffsetDateTime::now_utc();
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/getfilelink");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fileid"), Some(&"42".to_string()));
                assert_eq!(params.get("path"), None);
                assert_eq!(headers.len(), 0);

                Ok(FileLink {
                    path: "<path>".to_string(),
                    expires: utc_now.clone(),
                    hosts: vec!["host1".to_string(), "host2".to_string()],
                })
            });

        let filelink = client.getfilelink(input).await?;
        assert_eq!(filelink.path, "<path>".to_string());
        assert_eq!(filelink.expires, utc_now);
        assert_eq!(filelink.hosts, vec!["host1".to_string(), "host2".to_string()]);

        Ok(())
    }
}
