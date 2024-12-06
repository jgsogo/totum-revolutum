use crate::errors::DeserializationError;
use crate::models::{CurrenciesResponse, ErrorContent, ExchangeRatesResponse, UsageResponse};
use crate::{Error, Result};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use tracing::debug;

#[derive(Debug)]
pub struct OXRClient {
    app_id: String,
    http_client: reqwest::Client,
}

impl OXRClient {
    pub fn new(app_id: String) -> Result<Self> {
        let http_client = reqwest::ClientBuilder::new().build()?;
        Ok(Self { app_id, http_client })
    }

    fn build_url(endpoint: &str) -> String {
        format!("https://openexchangerates.org/api/{}", endpoint)
    }

    fn params(&self, mut params: HashMap<String, String>) -> HashMap<String, String> {
        params.insert("app_id".to_string(), self.app_id.clone());
        params
    }

    fn parse_text<T>(result: String) -> Result<T>
    where
        T: DeserializeOwned + 'static,
    {
        serde_json::from_str::<T>(&result).map_err(|e| {
            DeserializationError {
                string: result,
                source: e.into(),
            }
            .into()
        })
    }

    async fn get<T: DeserializeOwned + 'static>(&self, endpoint: &str, params: HashMap<String, String>) -> Result<T> {
        let url = Self::build_url(endpoint);
        debug!("GET '{url}'");
        let params = self.params(params);

        let request = self
            .http_client
            .get(url)
            // .headers(self.headers(headers))
            .query(&params);

        let r = request.send().await?;
        let maybe_err = r.error_for_status_ref().err();
        let text = r.text().await?;
        match maybe_err {
            Some(_e) => {
                let error = Self::parse_text::<ErrorContent>(text)?;
                Err(Error::OXRError(error))
            }
            None => Self::parse_text(text),
        }
    }

    pub async fn latest(&self, base: Option<&String>, symbols: Option<&String>) -> Result<ExchangeRatesResponse> {
        let mut params = HashMap::new();
        if let Some(base) = base {
            params.insert("base".to_string(), base.clone());
        }
        if let Some(symbols) = symbols {
            params.insert("symbols".to_string(), symbols.clone());
        }
        self.get("latest.json", params).await
    }

    pub async fn usage(&self) -> Result<UsageResponse> {
        self.get("usage.json", HashMap::new()).await
    }

    pub async fn historical(
        &self,
        date: &chrono::NaiveDate,
        base: Option<&String>,
        symbols: Option<&String>,
    ) -> Result<ExchangeRatesResponse> {
        let mut params = HashMap::new();
        if let Some(base) = base {
            params.insert("base".to_string(), base.clone());
        }
        if let Some(symbols) = symbols {
            params.insert("symbols".to_string(), symbols.clone());
        }

        let endpoint = format!("historical/{}.json", date);
        self.get(&endpoint, params).await
    }

    pub async fn currencies(&self) -> Result<CurrenciesResponse> {
        self.get("currencies.json", HashMap::new()).await
    }
}
