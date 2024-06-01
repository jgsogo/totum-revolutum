use crate::client::PCloudClient;
use crate::utils::http::BOUNDARY;
use crate::Result;
use async_trait::async_trait;
use headers::HeaderMapExt;
use http::HeaderMap;
use http_utils::rest::RESTClient;
use mime::Mime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::methods::fileops::FileDescriptor;
use crate::utils;

pub const ENDPOINT: &str = "/file_write";

#[derive(Serialize, Deserialize, Debug)]
pub struct FileWrite {
    pub bytes: u64,
}

#[async_trait]
pub trait PostFileWrite {
    async fn file_write(&self, descriptor: FileDescriptor, data: &[u8]) -> Result<FileWrite>;
}

#[async_trait]
impl<T: PCloudClient> PostFileWrite for T
where
    T: http_utils::HttpClient<Error = crate::Error>,
{
    async fn file_write(&self, descriptor: FileDescriptor, data: &[u8]) -> Result<FileWrite> {
        let mut headers = HeaderMap::default();
        let mime_multipart = Mime::from_str(&format!("multipart/form-data; boundary={BOUNDARY}")).unwrap();
        let content_type = headers::ContentType::from(mime_multipart);
        headers.typed_insert(content_type);

        let data = utils::http::create_file_write(&mut data.to_owned(), "filename")?;
        RESTClient::post(self, ENDPOINT, headers, &descriptor, data).await
    }
}

#[cfg(test)]
mod tests {

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_file_write() -> Result<()> {
        let mut client = MockLocalClient::new();
        let mut data = "gaudeamus igitur".as_bytes().to_vec();

        let bdata = utils::http::create_file_write(&mut data.clone(), "filename")?;
        client.expect_post().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &FileDescriptor, posted_data: Vec<u8>| {
                assert_eq!(endpoint, "/file_write");
                assert_eq!(params, &FileDescriptor::new(42));
                assert_eq!(posted_data, bdata);
                assert_eq!(headers.len(), 1);
                assert_eq!(
                    headers.get("content-type").unwrap(),
                    "multipart/form-data; boundary=ea3bbcf87c101592"
                );
                Ok(FileWrite { bytes: 10 })
            },
        );

        let r = client.file_write(FileDescriptor::new(42), &mut data).await?;
        assert_eq!(r.bytes, 10);
        Ok(())
    }
}
