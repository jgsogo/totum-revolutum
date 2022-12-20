use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use flume::Sender;
use tokio::time::Instant;
use tracing::{info, trace};

use pcloud_sdk::client::Client;
use pcloud_sdk::data::oauth2token::OAuth2TokenImpl;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::structures::Metadata;
use pcloud_sdk::types::{FolderID, PCloudFile};

use crate::diff::Filesystem;
use crate::local::LocalFileMetadata;
use crate::remote::file::RemoteFile;
use crate::remote::RemoteMetadata;

pub type PCloudHttpClient = pcloud_sdk::client::HttpClient<OAuth2TokenImpl>;

pub struct FilesystemPCloud<HttpClient: Client + Clone> {
    path: PathBuf,
    folderid: FolderID,
    pcloud: HttpClient,
}

impl<HttpClient: Client + Send + Sync + Clone> FilesystemPCloud<HttpClient> {
    pub async fn new(path: &Path, pcloud: HttpClient) -> Result<Self> {
        let listfolder_input = ListFolderInput::new_from_path(Some(path.to_str().unwrap().to_string()));
        let filtermeta = vec!["folderid"];
        let r = pcloud.listfolder_with_filtermeta(&listfolder_input, filtermeta).await?;

        Ok(Self {
            path: path.to_path_buf(),
            folderid: r
                .metadata
                .folderid
                .ok_or(anyhow!("Cannot get folderID for given path"))?,
            pcloud,
        })
    }

    #[allow(dead_code)]
    pub fn copy<T: LocalFileMetadata>(
        &self,
        _source: &T,
        _target: Option<RemoteMetadata>,
    ) -> Result<(&T, RemoteMetadata)> {
        todo!()
    }

    #[allow(dead_code)]
    pub fn rename(&self, _file: RemoteMetadata) -> Result<RemoteMetadata> {
        todo!()
    }

    fn work_on_contents(
        tx: flume::Sender<RemoteMetadata>,
        base_path: &Path,
        contents: &[Metadata],
        depth: usize,
    ) -> Result<()> {
        for it in contents.iter() {
            let path = base_path.join(Path::new(it.common.name.as_ref().unwrap()));
            trace!("{}{}", format!("{}|-- ", " ".repeat(depth * 4)), path.display());
            if !it.common.isfolder.unwrap() {
                let data: RemoteMetadata = (path, it.clone()).into();
                tx.send(data)?;
            } else {
                FilesystemPCloud::<HttpClient>::work_on_contents(
                    tx.clone(),
                    &path,
                    it.contents.as_ref().unwrap(),
                    depth + 1,
                )?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl<HttpClient: Client + Send + Sync + Clone> Filesystem for FilesystemPCloud<HttpClient> {
    type Metadata = RemoteMetadata;
    type File = RemoteFile<HttpClient>;

    fn root(&self) -> &Path {
        &self.path
    }

    async fn walk_directory(&self, tx: Sender<Self::Metadata>, _threads: usize) -> Result<()> {
        // FIXME: Here we can implement two different strategies. One of them is to iterate everything
        //  from the ROOT folder recursively, the other one is to list the files in each directory
        //  and use a thread pool to enter child directories and _recurse_.
        info!("Start remote visitor");
        let start = Instant::now();
        let mut list_folder_input = ListFolderInput::new_from_path(Some(self.root().to_str().unwrap().to_string()));
        list_folder_input.recursive = true;
        let filtermeta = vec!["name", "contents", "size", "hash", "isfolder"];
        let items = self
            .pcloud
            .listfolder_with_filtermeta(&list_folder_input, filtermeta)
            .await
            .unwrap();

        match &items.metadata.contents {
            Some(contents) => FilesystemPCloud::<HttpClient>::work_on_contents(tx, self.root(), contents, 0)?,
            None => (),
        }
        info!("Finished remote visitor in {:?}", start.elapsed());

        Ok(())
    }

    async fn create(&self, path: &Path) -> Result<Self::File> {
        match self.check_path(path) {
            Ok(v) => {
                let f = Self::File::new(PCloudFile::Path(v.to_str().unwrap().to_string()), self.pcloud.clone());
                // TODO: Open file to create/write/...
                Ok(f)
            }
            Err(e) => Err(e),
        }
    }

    async fn open(&self, path: &Path) -> Result<Self::File> {
        match self.check_path(path) {
            Ok(v) => {
                let f = Self::File::new(PCloudFile::Path(v.to_str().unwrap().to_string()), self.pcloud.clone());
                // TODO: Open the file with pcloud.popen... and use a file descriptor here
                Ok(f)
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tempfile::tempdir;

    use pcloud_sdk::error::Error;
    use pcloud_sdk::methods::folder::listfolder::ListFolder;
    use pcloud_sdk::mocks::client::MockLocalClient;

    use super::*;

    #[tokio::test]
    async fn test_root() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, MockLocalClient::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&"the/path".to_string()));
                assert_eq!(params.get("filtermeta"), Some(&"folderid".to_string()));

                Ok(ListFolder {
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        let fs = FilesystemPCloud::new(Path::new("the/path"), client).await?;
        assert_eq!(Path::new("the/path"), fs.root());
        Ok(())
    }

    #[tokio::test]
    async fn test_root_not_exists() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get::<ListFolder>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, MockLocalClient::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&"the/path".to_string()));
                assert_eq!(params.get("filtermeta"), Some(&"folderid".to_string()));

                Err(anyhow!(Error::ApiError {
                    code: 9999,
                    message: "Mock: the folder doesn't exist".to_string(),
                }))
            });

        let r = FilesystemPCloud::new(Path::new("the/path"), client).await;
        assert!(r.is_err());
        assert_eq!(
            r.err().unwrap().to_string(),
            "API error 9999: Mock: the folder doesn't exist".to_string()
        );
        Ok(())
    }

    //
    // #[test]
    // fn test_root_not_exists() {
    //     let tmp_dir = tempdir().unwrap();
    //     let r = FilesystemLocal::new(&tmp_dir.path().join("not-exist"));
    //     assert!(r.is_err());
    // }

    // use std::env;
    // use std::env::VarError;
    //
    //
    // use pcloud_sdk::data::oauth2token::OAuth2Token;
    //
    // use super::*;
    //
    // fn pcloud_client() -> Box<dyn pcloud_sdk::client::Client> {
    //     match env::var("PCLOUD_TEST_ACCESS_TOKEN") {
    //         Ok(access_token) => {
    //             let oauth2_token = OAuth2Token {
    //                 userid: 0,
    //                 locationid: 0,
    //                 access_token,
    //                 token_type: "bearer".to_string(),
    //             };
    //
    //             Box::new(pcloud_sdk::client::HttpClient::new(oauth2_token))
    //         }
    //         Err(_) => {}
    //     }
    // }
    //
    // #[test]
    // fn test_root_not_exists() -> Result<()> {
    //     let pcloud = pcloud_client();
    //     let tmp_dir = tempdir().unwrap();
    //     let r = FilesystemPCloud::new(&tmp_dir.path().join("not-exist"), pcloud);
    //     assert!(r.is_err());
    //     Ok(())
    // }

    /*
    #[test]
    fn test_root() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let fs = FilesystemLocal::new(tmp_dir.path())?;
        assert_eq!(fs::canonicalize(tmp_dir.path())?, fs.root());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_write_read() -> Result<()> {
        let tmp_dir = tempdir().unwrap();
        let fs = FilesystemLocal::new(tmp_dir.path())?;

        let filepath = tmp_dir.path().join("myfile");
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Create and write
        {
            let mut f = fs.create(&filepath).await?;
            f.write_all(&content).await?;
        }

        // Read
        {
            let mut file = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(content, content_read);
        }

        Ok(())
    }

     */
}
