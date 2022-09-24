use std::collections::HashMap;

use async_trait::async_trait;
use hyper::client::connect::Connect;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;

pub struct ListFolderInput {
    // path to the folder(discouraged)
    path: Option<String>,
    // id of the folder
    folderid: Option<i64>,

    // If is set full directory tree will be returned, which means that all directories will have contents filed.
    pub recursive: Option<u8>,
    // If is set, deleted files and folders that can be undeleted will be displayed.
    pub showdeleted: Option<u8>,
    // If is set, only the folder (sub)structure will be returned.
    pub nofiles: Option<u8>,
    // If is set, only user's own folders and files will be displayed.
    pub noshared: Option<u8>,
}

impl ListFolderInput {
    pub fn new_from_path(path: &str) -> ListFolderInput {
        ListFolderInput {
            path: Some(path.to_string()),
            folderid: None,
            recursive: None,
            showdeleted: None,
            nofiles: None,
            noshared: None,
        }
    }
    pub fn new_from_folderid(folderid: i64) -> ListFolderInput {
        ListFolderInput {
            path: None,
            folderid: Some(folderid),
            recursive: None,
            showdeleted: None,
            nofiles: None,
            noshared: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolder {
    pub metadata: Metadata,
}


#[async_trait]
pub trait GetListFolder<C>: client::Client<C>
    where
        C: Connect + Clone + Send + Sync + 'static
{
    async fn listfolder(&self, list_folder: &ListFolderInput) -> Result<ListFolder, hyper::Error>
        where
            C: Connect + Clone + Send + Sync + 'static
    {
        let url = format!("https://{}/listfolder", self.hostname());
        let mut params = HashMap::new();
        match list_folder {
            ListFolderInput { path: Some(p), .. } => {
                params.insert("path".to_string(), p.clone());
            }
            ListFolderInput { folderid: Some(f), .. } => {
                params.insert("folderid".to_string(), f.to_string());
            }
            _ => (),
        }

        let ret = self.get::<ListFolder>(&url, params).await?;
        Ok(ret)
    }
}

impl<C, T: client::Client<C>> GetListFolder<C> for T
    where
        C: Connect + Clone + Send + Sync + 'static {}
