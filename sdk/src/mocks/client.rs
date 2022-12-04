use async_trait::async_trait;
use mockall::mock;

use crate::client::Client;

mock! {
    pub LocalClient {}

    impl Clone for LocalClient {
        fn clone(&self) -> Self;
    }

     #[async_trait]
     impl Client for LocalClient {
        fn hostname(&self) -> String;

        fn access_token(&self) -> String;

        fn http_client(&self) -> reqwest::Client;
     }
}
