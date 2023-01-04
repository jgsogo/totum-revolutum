use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;
use crate::types::FolderID;

pub const ENDPOINT: &str = "/createfolderifnotexists";

pub enum CreateFolderIfNotExistsInput {
    Path(String),
    FolderAndName(FolderID, String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderIfNotExists {
    pub created: bool,
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetCreateFolderIfNotExists: client::Client {
    /// Creates the given directory (if it doesn't exists) and returns its metadata. It can only
    /// create one folder at a time, for nested ones you will need to call the function several
    /// times.
    async fn createfolderifnotexists(&self, input: &CreateFolderIfNotExistsInput) -> Result<CreateFolderIfNotExists> {
        let mut params = HashMap::new();
        match input {
            CreateFolderIfNotExistsInput::Path(p) => {
                params.insert("path".to_string(), p.clone());
            }
            CreateFolderIfNotExistsInput::FolderAndName(folderid, name) => {
                params.insert("folderid".to_string(), folderid.0.to_string());
                params.insert("name".to_string(), name.to_string());
            }
        }
        self.get::<CreateFolderIfNotExists>(ENDPOINT, params).await
    }
}

impl<T: client::Client> GetCreateFolderIfNotExists for T {}

#[cfg(test)]
mod tests {
    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_with_path() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/createfolderifnotexists");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("path"), Some(&"the/path/to/folder".to_string()));
                Ok(CreateFolderIfNotExists {
                    created: true,
                    metadata: Default::default(),
                })
            });
        let input = CreateFolderIfNotExistsInput::Path("the/path/to/folder".to_string());
        let r = client.createfolderifnotexists(&input).await?;
        assert_eq!(r.created, true);
        Ok(())
    }

    #[tokio::test]
    async fn test_with_folderid_and_name() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/createfolderifnotexists");
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"name.txt".to_string()));
                Ok(CreateFolderIfNotExists {
                    created: true,
                    metadata: Default::default(),
                })
            });
        let input = CreateFolderIfNotExistsInput::FolderAndName(FolderID(1234), "name.txt".to_string());
        let r = client.createfolderifnotexists(&input).await?;
        assert_eq!(r.created, true);
        Ok(())
    }
}
