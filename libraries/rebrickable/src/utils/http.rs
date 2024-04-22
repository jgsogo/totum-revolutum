use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use headers::HeaderMapExt;
use mime::Mime;
use reqwest;
use serde::de::DeserializeOwned;

use crate::error::Error;

const BOUNDARY: &str = "ea3bbcf87c101592";

fn create_response<T>(result: String) -> Result<T>
where
    T: DeserializeOwned,
{
    let r = serde_json::from_str::<T>(&result).map_err(|e| {
        anyhow!(Error::SerializationError {
            error: e,
            content: result.clone()
        })
    })?;

    Ok(r)

    /*
    match r.result {
        0 => match r.data {
            Some(data) => Ok(data),
            None => Err(anyhow!("Failed to parse data type from result string: {result}")),
        },
        _ => Err(anyhow!(Error::ApiError {
            code: r.result,
            message: r.error.unwrap_or_else(|| "Error message not available".into())
        })),
    }

         */
}

pub(crate) async fn get<T>(
    client: reqwest::Client,
    url: &str,
    //headers: HeaderMap,
    params: HashMap<String, String>,
) -> Result<T>
where
    T: DeserializeOwned,
{
    // TODO: Pass headers from the client, we don't need to keep-alive always, only in
    //  file_open/close/read/... or for performance reasons
    let header_map = {
        let mut header_map = headers::HeaderMap::new();
        let conn = headers::Connection::keep_alive();
        header_map.typed_insert(conn);
        header_map
    };

    let result = client
        .get(url)
        //.headers(headers)
        //.header("Keep-Alive", "timeout=5, max=1000")
        .headers(header_map)
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
    //headers: HeaderMap,
    params: HashMap<String, String>,
    data: Vec<u8>,
) -> Result<T>
where
    T: DeserializeOwned,
{
    let header_map = {
        let mut header_map = headers::HeaderMap::new();
        let conn = headers::Connection::keep_alive();
        header_map.typed_insert(conn);
        let mime_multipart = Mime::from_str(&format!("multipart/form-data; boundary={BOUNDARY}")).unwrap();
        let content_type = headers::ContentType::from(mime_multipart);
        header_map.typed_insert(content_type);
        header_map
    };

    let result = client
        .post(url)
        //.headers(headers)
        .headers(header_map)
        .query(&params)
        .body(reqwest::Body::from(data))
        .send()
        .await?
        .text()
        .await?;
    create_response(result)
}
