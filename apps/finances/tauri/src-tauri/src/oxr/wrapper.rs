use crate::{Error, Result};
use openexchangerates::OXRClient;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
/// By default, openexchangerates use USD as base currency
static DEFAULT_BASE_CCY: &str = "USD";

pub struct OXRWrapper {
    client: OXRClient,
    quotes: HashMap<(String, String), f32>,
}

impl OXRWrapper {
    pub fn new(client: OXRClient) -> Self {
        Self {
            client,
            quotes: HashMap::default(),
        }
    }

    async fn get_spot<'a>(&'a mut self, quoted: &str) -> Result<&'a f32> {
        let key = (DEFAULT_BASE_CCY.to_string(), quoted.to_string());
        match self.quotes.entry(key) {
            Entry::Vacant(entry) => {
                let default2quoted = self
                    .client
                    .latest(Some(&DEFAULT_BASE_CCY.to_string()), Some(&quoted.to_string()))
                    .await
                    .map_err(|e| Error::Other(e.to_string()))?;
                match default2quoted.rates.get(quoted) {
                    Some(rate) => Ok(entry.insert(*rate)),
                    None => Err(Error::Other("Error collecting the rate".to_string())),
                }
            }
            Entry::Occupied(o) => Ok(o.into_mut()),
        }
    }

    pub async fn fx_spot(&mut self, base: &str, quoted: &str) -> Result<f32> {
        let key = (base.to_string(), quoted.to_string());
        match self.quotes.get(&key) {
            Some(quote) => Ok(quote.clone()),
            None => {
                let default2base = self.get_spot(base).await?.clone();
                let default2quoted = self.get_spot(quoted).await?;
                let value = default2quoted / default2base;
                self.quotes.insert(key.clone(), value.clone());
                Ok(value)
            }
        }
    }
}
