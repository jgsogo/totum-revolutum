use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use mockall::mock;
use serde::de::DeserializeOwned;

use crate::client::Client;

mock! {
    #[allow(dead_code)]
    pub LocalClient {}

    #[allow(dead_code)]
    impl Clone for LocalClient {
        fn clone(&self) -> Self;
    }

    #[allow(dead_code)]
    #[async_trait]
    impl Client for LocalClient {
        async fn get<T>(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<T>
        where
            T: DeserializeOwned+ 'static;

        async fn post<T>(&self, endpoint: &str, mut params: HashMap<String, String>, data: Vec<u8>) -> Result<T>
        where
            T: DeserializeOwned+ 'static;
    }
}
