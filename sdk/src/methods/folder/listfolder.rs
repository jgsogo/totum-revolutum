use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::structures::Metadata;

#[derive(Default)]
pub struct ListFolderInput {
    // path to the folder(discouraged)
    path: Option<String>,
    // id of the folder
    folderid: Option<i64>,

    // If is set full directory tree will be returned, which means that all directories will have contents filed.
    pub recursive: bool,
    // If is set, deleted files and folders that can be undeleted will be displayed.
    pub showdeleted: Option<u8>,
    // If is set, only the folder (sub)structure will be returned.
    pub nofiles: Option<u8>,
    // If is set, only user's own folders and files will be displayed.
    pub noshared: Option<u8>,
}

impl ListFolderInput {
    pub fn new_from_path(path: Option<String>) -> ListFolderInput {
        ListFolderInput {
            path: Some(path.unwrap_or_else(|| "/".to_string())),
            ..Default::default()
        }
    }
    pub fn new_from_folderid(folderid: i64) -> ListFolderInput {
        ListFolderInput {
            folderid: Some(folderid),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolder {
    result: u8,
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetListFolder: client::Client {
    const ENDPOINT: &'static str = "/listfolder";

    async fn listfolder(&self, list_folder: &ListFolderInput) -> Result<ListFolder> {
        self.listfolder_with_filtermeta(list_folder, vec![]).await
    }

    async fn listfolder_with_filtermeta(
        &self,
        list_folder: &ListFolderInput,
        filtermeta: Vec<&str>,
    ) -> Result<ListFolder> {
        let mut params = HashMap::new();
        match list_folder {
            ListFolderInput { path: Some(p), .. } => {
                params.insert("path".to_string(), p.clone());
            }
            ListFolderInput { folderid: Some(f), .. } => {
                params.insert("folderid".to_string(), f.to_string());
            }
            _ => todo!("Either path or folderid is compulsory"),
        }

        if list_folder.recursive {
            params.insert("recursive".to_string(), "1".to_string());
        }

        let filtermeta = filtermeta.join(",");
        params.insert("filtermeta".to_string(), filtermeta);

        let ret = self.get::<ListFolder>(Self::ENDPOINT, params).await?;

        ret.result
        Gestionar los result, pasarlo a un método común en client::Client

        Ok(ret)
    }
}

impl<T: client::Client> GetListFolder for T {}
