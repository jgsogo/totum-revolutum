use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use camino::Utf8Path;
use headers::HeaderMapExt;
use reqwest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::{DeserializationError, DeserializationErrorKind, PCloudError, Result};

pub const BOUNDARY: &str = "ea3bbcf87c101592";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ApiResult<T> {
    pub result: u16,
    pub error: Option<String>,

    #[serde(flatten)]
    pub data: Option<T>,
}

pub fn create_response<T>(result: String) -> Result<T>
where
    T: DeserializeOwned,
{
    match serde_json::from_str::<ApiResult<T>>(&result) {
        Ok(r) => match r.result {
            0 => match r.data {
                Some(data) => Ok(data),
                None => Err(DeserializationError {
                    string: result,
                    source: DeserializationErrorKind::EmptyDataField,
                }
                .into()),
            },
            _ => Err(PCloudError::from((r.result, r.error)).into()),
        },
        Err(e) => Err(DeserializationError {
            string: result,
            source: e.into(),
        }
        .into()),
    }
}

pub(crate) async fn get_bytes(client: reqwest::Client, url: &str, params: HashMap<String, String>) -> Result<Vec<u8>> {
    let header_map = {
        let mut header_map = headers::HeaderMap::new();
        let conn = headers::Connection::keep_alive();
        header_map.typed_insert(conn);
        header_map
    };

    let r = client
        .get(url)
        .headers(header_map)
        .query(&params)
        .send()
        .await?
        .bytes()
        .await?;

    // If there is an error, it returns a JSON with the result and error fields
    let as_str = String::from_utf8_lossy(&r);
    if let Ok(r) = serde_json::from_str::<ApiResult<()>>(&as_str) {
        Err(PCloudError::from((r.result, r.error)).into())
    } else {
        // If not, just the bytes
        Ok(r.to_vec())
    }
}

pub(crate) fn create_file_data(local_filepath: &Utf8Path, filename: &str) -> io::Result<Vec<u8>> {
    // FIXME: Receive BOUNDARY as argument
    let mut data = Vec::new();
    write!(data, "--{BOUNDARY}\r\n")?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"smfile\"; filename=\"{filename}\"\r\n"
    )?;
    write!(data, "\r\n")?;

    let mut f = File::open(local_filepath)?;
    f.read_to_end(&mut data)?;

    write!(data, "\r\n")?;
    write!(data, "--{BOUNDARY}--\r\n")?;

    Ok(data)
}

/// Creates the payload for a POST request (`form-data`) to send the contents of a file
pub fn create_file_write(content: &mut Vec<u8>, filename: &str) -> io::Result<Vec<u8>> {
    // FIXME: Receive BOUNDARY as argument
    let mut data = Vec::new();
    write!(data, "--{BOUNDARY}\r\n")?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"files\"; filename=\"{filename}\"\r\n"
    )?;
    // write!(data, "Content-Type: application/octet-stream\r\n")?;
    // write!(data, "Content-Transfer-Encoding: binary\r\n")?;
    write!(data, "\r\n")?;
    data.append(content);

    write!(data, "\r\n")?;
    write!(data, "--{BOUNDARY}--\r\n")?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;

    use super::*;
    use crate::error::PCLOUDERROR_MESSAGE_NOT_AVAILABLE;
    use crate::methods::general::UserInfo;
    use crate::Error;

    #[test]
    fn create_response_success() -> Result<()> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("userinfo.json");
        let file = File::open(userinfo_json).unwrap();
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let r = create_response::<UserInfo>(content);
        assert!(r.is_ok());
        Ok(())
    }

    #[test]
    fn create_response_serialization_error() {
        let r = create_response::<UserInfo>("this is not serializable".into());
        assert!(r.is_err());
        assert!(matches!(
            r.unwrap_err(),
            Error::DeserializationError(DeserializationError {
                source: DeserializationErrorKind::SerdeError { .. },
                string: ref msg,
            })if msg == "this is not serializable"
        ));
    }

    #[test]
    fn create_response_api_error() {
        let r = create_response::<UserInfo>("{\"result\": 1234, \"error\": \"message\"}".into());
        assert!(r.is_err());
        assert!(matches!(
            r.unwrap_err(),
            Error::PCloudError(PCloudError::UnclassifiedError {
                code: 1234,
                message: ref msg,
            })if msg == "message"
        ));
    }

    #[test]
    fn create_response_api_error_no_message() {
        let r = create_response::<UserInfo>("{\"result\": 1234}".into());
        assert!(r.is_err());
        assert!(matches!(
            r.unwrap_err(),
            Error::PCloudError(PCloudError::UnclassifiedError {
                code: 1234,
                message: ref msg,
            })if msg == PCLOUDERROR_MESSAGE_NOT_AVAILABLE
        ));
    }
}
