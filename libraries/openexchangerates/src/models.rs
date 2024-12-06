use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Deserialize, Debug, Error)]
pub struct ErrorContent {
    pub error: bool,
    pub status: u16,
    pub message: String,
    pub description: String,
}

impl fmt::Display for ErrorContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.description)
    }
}

#[derive(Deserialize, Debug)]
pub struct ExchangeRatesResponse {
    pub disclaimer: String,
    pub license: String,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
    pub rates: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
pub struct CurrenciesResponse {
    #[serde(flatten)]
    pub currencies: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct UsageResponse {
    pub status: u16,
    pub data: UsageData,
}

#[derive(Deserialize, Debug)]
pub struct UsageData {
    pub app_id: String,
    pub status: String,
    pub plan: Plan,
    pub usage: Usage,
}

#[derive(Deserialize, Debug)]
pub struct Plan {
    pub name: String,
    pub quota: String,
    pub update_frequency: String,
    pub features: Features,
}

#[derive(Deserialize, Debug)]
pub struct Features {
    pub base: bool,
    pub symbols: bool,
    pub experimental: bool,
    #[serde(rename = "time-series")]
    pub time_series: bool,
    pub convert: bool,
    #[serde(rename = "bid-ask")]
    pub bid_ask: bool,
    pub ohlc: bool,
    pub spot: bool,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub requests: u16,
    pub requests_quota: u16,
    pub requests_remaining: u16,
    pub days_elapsed: u16,
    pub days_remaining: u16,
    pub daily_average: u16,
}
