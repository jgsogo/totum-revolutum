use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use flume::Sender;
use tokio::time::Instant;
use tracing::{info, trace};

use pcloud_sdk::client::Client;
use pcloud_sdk::data::oauth2token::OAuth2TokenImpl;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::structures::Metadata;
use pcloud_sdk::types::FolderID;

use crate::diff::{File, Filesystem};
use crate::local::LocalFileMetadata;
use crate::remote::file::RemoteFile;
use crate::remote::RemoteMetadata;

pub type PCloudHttpClient = pcloud_sdk::client::HttpClient<OAuth2TokenImpl>;

pub struct FilesystemPCloud<HttpClient: Client + Clone> {
    path: PathBuf,
    folderid: FolderID,
    // TODO: This shouldn't be an `Arc<HttpClient>`. It should be just `HttpClient`
    pcloud: Arc<HttpClient>,
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
                .ok_or_else(|| anyhow!("Cannot get folderID for given path"))?,
            pcloud: Arc::new(pcloud),
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
        tx: Sender<RemoteMetadata>,
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
impl<HttpClient: Client + Send + Sync + Clone + 'static> Filesystem for FilesystemPCloud<HttpClient> {
    type Metadata = RemoteMetadata;

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

    async fn create(&self, path: &Path) -> Result<Box<dyn File>> {
        match self.check_path(path) {
            Ok(v) => {
                let relative_path = v.strip_prefix(self.root())?;
                let fd = self
                    .pcloud
                    .file_open(
                        Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC,
                        FileOpenPath::FolderAndName(self.folderid.clone(), relative_path.to_string_lossy().parse()?),
                    )
                    .await?;

                let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone());
                Ok(Box::new(f))
            }
            Err(e) => Err(e),
        }
    }

    async fn open(&self, path: &Path) -> Result<Box<dyn File>> {
        match self.check_path(path) {
            Ok(v) => {
                //let relative_path = v.strip_prefix(self.root())?;
                let fd = self
                    .pcloud
                    .file_open(Flags::empty(), FileOpenPath::Path(v.to_string_lossy().parse()?))
                    .await?;

                let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone());
                Ok(Box::new(f))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pcloud_sdk::error::Error;
    use pcloud_sdk::methods::fileops::file_open::FileOpen;
    use pcloud_sdk::methods::fileops::file_write::FileWrite;
    use pcloud_sdk::methods::fileops::{file_open, file_read, file_write};
    use pcloud_sdk::methods::folder::listfolder;
    use pcloud_sdk::methods::folder::listfolder::ListFolder;
    use pcloud_sdk::mocks::client::MockLocalClient;
    use pcloud_sdk::types::FileID;
    use pcloud_sdk::utils;

    use super::*;

    #[tokio::test]
    async fn test_root() -> Result<()> {
        let mut client = MockLocalClient::new();
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
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
                assert_eq!(endpoint, listfolder::ENDPOINT);
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

    #[tokio::test]
    async fn test_create_write_all() -> Result<()> {
        let mut client = MockLocalClient::new();

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
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

        // Expectation for create
        client
            .expect_get::<FileOpen>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_open::ENDPOINT);
                assert_eq!(params.len(), 3);
                let flags = (Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC).bits().to_string();
                assert_eq!(params.get("flags"), Some(&flags));
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"file".to_string()));
                Ok(FileOpen {
                    fd: 42,
                    fileid: FileID(1234),
                })
            });

        // Expectation for write_all
        client
            .expect_post()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>, posted_data: Vec<u8>| {
                assert_eq!(endpoint, file_write::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                let mut data = b"Hello, world!".to_vec();
                let bdata = utils::http::file_write(&mut data, "filename")?;
                assert_eq!(posted_data, bdata);
                Ok(FileWrite { bytes: 321 })
            });

        // Create the filesystem
        let fs = FilesystemPCloud::new(Path::new("the/path"), client).await?;

        let filepath = Path::new("file");
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Create and write_all
        {
            let mut f = fs.create(&filepath).await?;
            f.write_all(&content).await?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_open_read_all() -> Result<()> {
        let mut client = MockLocalClient::new();

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
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

        // Expectation for open
        client
            .expect_get::<FileOpen>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_open::ENDPOINT);
                assert_eq!(params.len(), 2);
                let flags = (Flags::empty()).bits().to_string();
                assert_eq!(params.get("flags"), Some(&flags));
                assert_eq!(params.get("path"), Some(&"the/path/file".to_string()));
                Ok(FileOpen {
                    fd: 42,
                    fileid: FileID(1234),
                })
            });

        // Expectation for read_to_end
        client
            .expect_get_bytes()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_read::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                assert_eq!(params.get("count"), Some(&"100".to_string()));
                Ok(b"Hello, world!".to_vec())
            });

        // Create the filesystem
        let fs = FilesystemPCloud::new(Path::new("the/path"), client).await?;

        let filepath = Path::new("file");
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Open and read
        {
            let mut file = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(content, content_read);
        }

        Ok(())
    }
}
