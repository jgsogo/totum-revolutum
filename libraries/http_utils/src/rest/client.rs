use crate::{AddToParams, HttpClient};
use async_trait::async_trait;
use headers::HeaderMap;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait RESTClient: HttpClient<Error = Self::RESTClientError> {
    type RESTClientError: From<reqwest::Error> + From<serde_json::Error>;

    /// Deserialize API call result to return type
    fn parse_response<T>(result: String) -> std::result::Result<T, Self::Error>
    where
        T: DeserializeOwned + 'static,
    {
        serde_json::from_str::<T>(&result).map_err(|e| e.into())
    }

    /// Runs GET request to the given `endpoint` (URL will be built using [`self.build_url`]) with
    /// some `params`
    async fn get<T, TParams: AddToParams + Sync + 'static>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        params: &TParams,
    ) -> std::result::Result<T, Self::Error>
    where
        T: DeserializeOwned + 'static,
    {
        let response = HttpClient::get(self, endpoint, headers, params).await?;
        Self::parse_response(response.text().await?)
    }

    async fn post<T, TParams: AddToParams + Sync + 'static>(
        &self,
        endpoint: &str,
        headers: HeaderMap,
        params: &TParams,
        data: Vec<u8>,
    ) -> std::result::Result<T, Self::Error>
    where
        T: DeserializeOwned + 'static,
    {
        let response = HttpClient::post(self, endpoint, headers, params, data).await?;
        Self::parse_response(response.text().await?)
    }
}
