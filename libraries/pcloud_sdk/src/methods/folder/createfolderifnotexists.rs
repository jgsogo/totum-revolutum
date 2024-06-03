use std::collections::HashMap;

use async_trait::async_trait;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use http_utils::rest::RESTClient;
use http_utils::AddToParams;

use crate::client::PCloudClient;
use crate::structures::MetadataFolder;
use crate::types::FolderID;
use crate::Result;

pub const ENDPOINT: &str = "/createfolderifnotexists";

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderIfNotExists {
    pub created: Option<bool>,
    pub metadata: MetadataFolder,
}

#[async_trait]
pub trait GetCreateFolderIfNotExists {
    /// Creates the given directory (if it doesn't exist) and returns its metadata
    ///
    /// Only the `folderid`+`name` alternative is implemented as it's the one recommended in the
    /// documentation.
    ///
    /// Link: <https://docs.pcloud.com/methods/folder/createfolderifnotexists.html>
    async fn createfolderifnotexists(&self, folder_id: &FolderID, name: &str) -> Result<CreateFolderIfNotExists>;
}

#[async_trait]
impl<T: PCloudClient> GetCreateFolderIfNotExists for T {
    async fn createfolderifnotexists(&self, folder_id: &FolderID, name: &str) -> Result<CreateFolderIfNotExists> {
        let mut params = HashMap::new();
        folder_id.add_to_params(&mut params);
        params.insert("name".to_string(), name.to_string());

        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &params).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_with_folderid_and_name() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, "/createfolderifnotexists");
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"name.txt".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(CreateFolderIfNotExists {
                    created: Some(true),
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });
        let r = client.createfolderifnotexists(&FolderID::new(1234), "name.txt").await?;
        assert_eq!(r.created.unwrap(), true);
        Ok(())
    }
}
