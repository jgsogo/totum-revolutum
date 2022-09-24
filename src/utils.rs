use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use http::{Method, Request, Uri};
use hyper;
use hyper::client::connect::Connect;
use hyper::header::CONTENT_TYPE;
use hyper::Body;
use serde::de::DeserializeOwned;
use url::Url;

const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

fn url_with_params(url: &str, params: HashMap<String, String>) -> Uri {
    let mut url = Url::parse(url).unwrap();
    for (key, value) in params {
        url.query_pairs_mut().append_pair(&key, &value);
    }
    url.as_str().parse().unwrap()
}

async fn execute_request<C, T>(
    client: hyper::Client<C>,
    req: Request<Body>,
) -> Result<T, hyper::Error>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: DeserializeOwned,
{
    let resp = client.request(req).await?;
    let bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let result = String::from_utf8(bytes.into_iter().collect()).expect("");
    let deserialized: T = match serde_json::from_str(&result) {
        Ok(data) => data,
        Err(e) => {
            // TODO: Provide enough information to debug, but also return meaningful error
            panic!(
                "Cannot parse '{}' into {}",
                result,
                std::any::type_name::<T>()
            );
        }
    };
    Ok(deserialized)
}

pub(crate) async fn get<C, T>(
    client: hyper::Client<C>,
    url: &str,
    params: HashMap<String, String>,
) -> Result<T, hyper::Error>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: DeserializeOwned,
{
    let uri = url_with_params(url, params);
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::empty())
        .unwrap();
    execute_request(client, req).await
}

pub(crate) async fn post<C, T>(
    client: hyper::Client<C>,
    url: &str,
    params: HashMap<String, String>,
    data: Vec<u8>,
) -> Result<T, hyper::Error>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: DeserializeOwned,
{
    let uri = url_with_params(url, params);
    let req = Request::builder()
        .method(Method::POST)
        .uri(uri.clone())
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={BOUNDARY}"),
        )
        .body(data.into())
        .unwrap();
    execute_request(client, req).await
}

pub(crate) fn file_data(localfile: String, filename: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    write!(data, "--{}\r\n", BOUNDARY)?;
    write!(
        data,
        "{}",
        format!(
            "Content-Disposition: form-data; name=\"smfile\"; filename=\"{}\"\r\n",
            filename
        )
    )?;
    write!(data, "\r\n")?;

    let mut f = File::open(localfile)?;
    f.read_to_end(&mut data)?;

    write!(data, "\r\n")?;
    write!(data, "--{}--\r\n", BOUNDARY)?;

    Ok(data)
}
