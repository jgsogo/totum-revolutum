use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::client;

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
impl<T: client::Client> GetSetDetail for T {
    async fn sets_detail(&self, set_num: String) -> Result<SetDetail> {
        let endpoint = format!("/api/v3/lego/sets/{set_num}");
        self.get::<SetDetail>(&endpoint, HashMap::new()).await
    }
}
