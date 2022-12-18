use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use anyhow::{anyhow, Result};
use hyper;
use hyper::header::CONTENT_TYPE;
use reqwest;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::Error;

const BOUNDARY: &str = "------------------------ea3bbcf87c101592";

#[derive(Serialize, Deserialize, Debug)]
struct ApiResult<T> {
    result: u16,
    error: Option<String>,

    #[serde(flatten)]
    data: Option<T>,
}

fn create_response<T>(result: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let r = serde_json::from_str::<ApiResult<T>>(&result).map_err(|e| {
        anyhow!(Error::SerializationError {
            error: e,
            content: result
        })
    })?;

    match r.result {
        0 => Ok(r.data.unwrap()),
        _ => Err(anyhow!(Error::ApiError {
            code: r.result,
            message: r.error.unwrap_or("Error message not available".into())
        })),
    }
}

pub(crate) async fn get<T>(client: reqwest::Client, url: &str, params: HashMap<String, String>) -> Result<T>
where
    T: DeserializeOwned,
{
    let result = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .query(&params)
        .send()
        .await?
        .text()
        .await?;
    create_response(result)
}

pub(crate) async fn post<T>(
    client: reqwest::Client,
    url: &str,
    params: HashMap<String, String>,
    data: Vec<u8>,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let result = client
        .post(url)
        .header(CONTENT_TYPE, format!("multipart/form-data; boundary={BOUNDARY}"))
        .query(&params)
        .body(reqwest::Body::from(data))
        .send()
        .await?
        .text()
        .await?;
    create_response(result)
}

pub(crate) fn file_data(localfile: String, filename: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(data, "--{}\r\n", BOUNDARY)?;
    write!(
        data,
        "Content-Disposition: form-data; name=\"smfile\"; filename=\"{filename}\"\r\n"
    )?;
    write!(data, "\r\n")?;

    let mut f = File::open(localfile)?;
    f.read_to_end(&mut data)?;

    write!(data, "\r\n")?;
    write!(data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::{read, File};
    use std::io::BufReader;
    use std::path::Path;

    use crate::error::Error::SerializationError;
    use crate::methods::general::UserInfo;

    use super::*;

    #[test]
    fn create_response_success() -> Result<()> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
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
        assert!(r.unwrap_err().to_string().contains("Serialization error"));
    }

    #[test]
    fn create_response_api_error() {
        let r = create_response::<UserInfo>("{\"result\": 1234, \"error\": \"message\"}".into());
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().to_string(), "API error 1234: message".to_string());
    }

    #[test]
    fn create_response_api_error_no_message() {
        let r = create_response::<UserInfo>("{\"result\": 1234}".into());
        assert!(r.is_err());
        assert_eq!(
            r.unwrap_err().to_string(),
            "API error 1234: Error message not available".to_string()
        );
    }
}
