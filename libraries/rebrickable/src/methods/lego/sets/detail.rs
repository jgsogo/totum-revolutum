use std::collections::HashMap;

use crate::client::RebrickableClient;
use crate::Result;
use async_trait::async_trait;
use http::HeaderMap;
use http_utils::rest::RESTClient;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct SetDetail {
    pub set_num: String,
    pub name: String,
    pub year: u16,
    pub theme_id: u8,
    pub num_parts: u16,
    pub set_img_url: String,
    pub set_url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub last_modified_dt: OffsetDateTime,
}

#[async_trait]
pub trait GetSetDetail {
    async fn sets_detail(&self, set_num: String) -> Result<SetDetail>;
}

#[async_trait]
impl<T: RebrickableClient> GetSetDetail for T {
    async fn sets_detail(&self, set_num: String) -> Result<SetDetail> {
        let endpoint = format!("/lego/sets/{set_num}");
        RESTClient::get(self, &endpoint, HeaderMap::default(), &HashMap::new()).await
    }
}
