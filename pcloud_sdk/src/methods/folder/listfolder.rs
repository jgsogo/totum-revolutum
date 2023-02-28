use anyhow::Result;
use async_trait::async_trait;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::client;
use crate::methods::params::{Params, ParamsType};
use crate::structures::Metadata;
use crate::types::Folder;

pub const ENDPOINT: &str = "/listfolder";

pub struct ListFolderInput {
    /// ID of the folder, or path to it (discouraged)
    folder: Folder,

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
    pub fn new(folder: Folder) -> ListFolderInput {
        ListFolderInput {
            folder,
            recursive: false,
            showdeleted: None,
            nofiles: None,
            noshared: None,
        }
    }
}

impl Params for ListFolderInput {
    fn add_to_params(&self, params: &mut ParamsType) -> Result<()> {
        self.folder.add_to_params(params)?;
        if self.recursive {
            params.insert("recursive".to_string(), "1".to_string());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolder {
    pub metadata: Metadata,
}

#[async_trait]
pub trait GetListFolder {
    async fn listfolder(&self, list_folder: ListFolderInput) -> Result<ListFolder> {
        self.listfolder_with_filtermeta(list_folder, vec![]).await
    }

    async fn listfolder_with_filtermeta(
        &self,
        list_folder: ListFolderInput,
        filtermeta: Vec<&str>,
    ) -> Result<ListFolder>;
}

#[async_trait]
impl<T: client::Client> GetListFolder for T {
    async fn listfolder_with_filtermeta(
        &self,
        list_folder: ListFolderInput,
        filtermeta: Vec<&str>,
    ) -> Result<ListFolder> {
        let mut params = list_folder.into_params()?;
        if !filtermeta.is_empty() {
            // We insert `id` always to prevent a pcloud API bug. If we only use one element,
            // for example `filtermeta=folderid`, the response JSON is not well formed when there
            // are files and folders inside the query directory, it returns
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

#[cfg(test)]
mod tests {
    use camino::Utf8Path;
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use crate::types::FolderID;
    use crate::utils::http::ApiResult;

    use super::*;

    #[test]
    fn test_deserialize_with_filtermeta() {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let userinfo_json = Utf8Path::new(&manifest_dir)
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
