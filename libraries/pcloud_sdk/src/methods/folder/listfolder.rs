use crate::client::PCloudClient;
use crate::structures::MetadataFolder;
use crate::types::Folder;
use crate::Result;
use async_trait::async_trait;
use collections::HashMap;
use http::HeaderMap;
use http_utils::rest::RESTClient;
use http_utils::AddToParams;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections;

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

impl AddToParams for ListFolderInput {
    fn add_to_params(&self, params: &mut HashMap<String, String>) {
        self.folder.add_to_params(params);
        if self.recursive {
            params.insert("recursive".to_string(), "1".to_string());
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFolder {
    pub metadata: MetadataFolder,
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
impl<T: PCloudClient> GetListFolder for T {
    async fn listfolder_with_filtermeta(
        &self,
        list_folder: ListFolderInput,
        filtermeta: Vec<&str>,
    ) -> Result<ListFolder> {
        let mut params = HashMap::new();
        list_folder.add_to_params(&mut params);
        let mut filtermeta = filtermeta;

        // TODO: I'm afraid not all the fields are valid here... search some docs or try/error and
        // TODO: document them manually (and raise if any of them is used)

        // Insert `folderid` and `fileid` always, it is required to parse [`MetadataFolder`] and
        // differentiate it from [`MetadataFile`]. Read about `#[serde(untagged)]` in [`Metadata`]
        // enum for more info about why this is needed.
        filtermeta.push("folderid");
        filtermeta.push("fileid");

        // Having two elements is also required to prevent a pcloud API bug. If we only use one
        // element, for example `filtermeta=folderid`, the response JSON is not well formed when
        // there are files and folders inside the query directory, it returns some empty lists
        // where empty dictionaries were expected

        let filtermeta = filtermeta.into_iter().unique().collect::<Vec<_>>().join(",");
        params.insert("filtermeta".to_string(), filtermeta);
        RESTClient::get(self, ENDPOINT, HeaderMap::default(), &params).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::BufReader;

    use camino::Utf8Path;

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
                assert_eq!(data.metadata.folderid, FolderID::new(4075092622));
            }
        }
    }
}
