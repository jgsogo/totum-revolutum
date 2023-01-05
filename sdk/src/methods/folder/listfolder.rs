use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use itertools::Itertools;

use crate::client;
use crate::structures::Metadata;

pub const ENDPOINT: &str = "/listfolder";

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
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetListFolder: client::Client {
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

        if !filtermeta.is_empty() {
            // We insert `id` always to prevent a pcloud API bug. If we only use one element,
            // for example `filtermeta=folderid`, the response JSON is not well formed, it returns
            // some empty lists where empty dictionaries were expected
            let mut filtermeta = filtermeta;
            filtermeta.push("id");
            let filtermeta = filtermeta.into_iter().unique().collect::<Vec<_>>().join(",");
            params.insert("filtermeta".to_string(), filtermeta);
        }

        let ret = self.get::<ListFolder>(ENDPOINT, params).await?;

        Ok(ret)
    }
}

impl<T: client::Client> GetListFolder for T {}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    use crate::types::FolderID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Path::new(&manifest_dir)
            .join("resources")
            .join("testdata")
            .join("listfolder_filtermeta_folderid.json");
        let file = File::open(userinfo_json).unwrap();
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, ApiResult<ListFolder>>(reader) {
            Err(e) => panic!("Error reading the file: {e}"),
            Ok(data) => {
                assert_eq!(data.result, 0);
                let data = data.data.unwrap();
                assert_eq!(data.metadata.folderid.unwrap(), FolderID(4075092622));
            }
        }
    }
}
