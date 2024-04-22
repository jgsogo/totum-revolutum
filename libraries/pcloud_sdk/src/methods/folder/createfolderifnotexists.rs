use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8PathBuf;
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::client::PCloudClient;
use http_utils::rest::RESTClient;

use crate::methods::params::{Params, ParamsType};
use crate::structures::MetadataFolder;
use crate::types::FolderID;

pub const ENDPOINT: &str = "/createfolderifnotexists";

pub enum TargetFolder {
    FolderAndName((FolderID, String)),
    Path(Utf8PathBuf),
}

impl Params for TargetFolder {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        match &self {
            TargetFolder::Path(p) => {
                params.insert("path".to_string(), p.to_string());
            }
            TargetFolder::FolderAndName((folderid, name)) => {
                params.insert("folderid".to_string(), folderid.0.to_string());
                params.insert("name".to_string(), name.clone());
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFolderIfNotExists {
    pub created: Option<bool>,
    pub metadata: MetadataFolder,
}

#[async_trait]
pub trait GetCreateFolderIfNotExists {
    /// Creates the given directory (if it doesn't exists) and returns its metadata. It can only
    /// create one folder at a time, for nested ones you will need to call the function several
    /// times.
    async fn createfolderifnotexists(&self, input: TargetFolder) -> Result<CreateFolderIfNotExists>;
}

#[async_trait]
impl<T: PCloudClient> GetCreateFolderIfNotExists for T {
    async fn createfolderifnotexists(&self, input: TargetFolder) -> Result<CreateFolderIfNotExists> {
        RESTClient::get::<CreateFolderIfNotExists>(self, ENDPOINT, HeaderMap::default(), input.into_params()?).await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_with_path() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/createfolderifnotexists");
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("path"), Some(&"the/path/to/folder".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(CreateFolderIfNotExists {
                    created: Some(true),
                    metadata: MetadataFolder::default(FolderID(1234)),
                })
            });
        let input = TargetFolder::Path(Utf8PathBuf::from_str("the/path/to/folder")?);
        let r = client.createfolderifnotexists(input).await?;
        assert_eq!(r.created.unwrap(), true);
        Ok(())
    }

    #[tokio::test]
    async fn test_with_folderid_and_name() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(|endpoint, headers: HeaderMap, params: HashMap<_, _>| {
                assert_eq!(endpoint, "/createfolderifnotexists");
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"name.txt".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(CreateFolderIfNotExists {
                    created: Some(true),
                    metadata: MetadataFolder::default(FolderID(1234)),
                })
            });
        let input = TargetFolder::FolderAndName((FolderID(1234), "name.txt".to_string()));
        let r = client.createfolderifnotexists(input).await?;
        assert_eq!(r.created.unwrap(), true);
        Ok(())
    }
}
