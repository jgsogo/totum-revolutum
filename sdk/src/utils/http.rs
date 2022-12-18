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

use crate::error::Error;

const BOUNDARY: &str = "------------------------ea3bbcf87c101592";

fn create_response<T>(result: String) -> Result<T>
where
    T: DeserializeOwned,
{
    match serde_json::from_str(&result) {
        Ok(data) => Ok(data),
        Err(e) => {
            // TODO: Provide enough information to debug, but also return meaningful error
            Err(anyhow!(
                "Serialization error {} from result {}",
                Error::SerializationError(e),
                result
            ))
            /*
            Err(Box::new(Error::APIError(&format!(
                "Cannot parse '{}' into {}",
                result,
                std::any::type_name::<T>()
            ))))
            */
        }
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
