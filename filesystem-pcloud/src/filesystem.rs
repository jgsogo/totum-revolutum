use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use flume::Sender;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tracing::{info, trace, warn};

use pcloud_sdk::client::Client;
use pcloud_sdk::methods::file::deletefile::{DeleteFileInput, GetDeleteFile};
use pcloud_sdk::methods::file::stat::{GetStat, StatInput};
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::fileops::FileDescriptor;
use pcloud_sdk::methods::folder::createfolderifnotexists::{CreateFolderIfNotExistsInput, GetCreateFolderIfNotExists};
use pcloud_sdk::methods::folder::deletefolder::{DeleteFolderInput, GetDeleteFolder};
use pcloud_sdk::methods::folder::deletefolderrecursive::{DeleteFolderRecursiveInput, GetDeleteFolderRecursive};
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::methods::oauth2::OAuth2TokenImpl;
use pcloud_sdk::structures::Metadata;
use pcloud_sdk::types::FolderID;

use crate::file::RemoteFile;
use crate::RemoteMetadata;
use filesystem::{File, Filesystem};

pub type PCloudHttpClient = pcloud_sdk::client::HttpClient<OAuth2TokenImpl>;

pub enum FileCloseMessage {
    FileDescriptor(FileDescriptor),
    Stop,
}

pub struct FilesystemPCloud<HttpClient: Client + Clone> {
    path: PathBuf,
    folderid: FolderID,
    // TODO: This shouldn't be an `Arc<HttpClient>`. It should be just `HttpClient`
    pcloud: Arc<HttpClient>,

    tx_file_close: Sender<FileCloseMessage>,
    thread_file_close: Option<JoinHandle<()>>,
}

async fn get_folderid<HttpClient: Client + Send + Sync + Clone>(
    pcloud: &Arc<HttpClient>,
    path: &Path,
) -> Result<FolderID> {
    let listfolder_input = ListFolderInput::new_from_path(Some(path.to_str().unwrap().to_string()));
    let filtermeta = vec!["folderid"];
    let r = pcloud.listfolder_with_filtermeta(&listfolder_input, filtermeta).await?;
    r.metadata
        .folderid
        .ok_or_else(|| anyhow!("Cannot get folderID for given path"))
}

impl<HttpClient: Client + Send + Sync + Clone + 'static> FilesystemPCloud<HttpClient> {
    pub async fn new(path: &Path, pcloud: HttpClient) -> Result<Self> {
        let pcloud = Arc::new(pcloud);
        let folderid = get_folderid(&pcloud, path).await?;
        let (tx, rx) = flume::unbounded::<FileCloseMessage>();

        // This async loop will take care of calling the 'file_close' method when RemoteFiles go out
        //  of scope. Here we can call this async method, while it is not possible to do it in the
        //  `Drop` implementation of those files.
        let pcloud_clone = pcloud.clone();
        let t = tokio::spawn(async move {
            while let Ok(msg) = rx.recv_async().await {
                match msg {
                    FileCloseMessage::FileDescriptor(fd) => {
                        if let Err(e) = pcloud_clone.file_close(fd).await {
                            warn!("Error closing file '{fd}': {e}");
                        }
                    }
                    FileCloseMessage::Stop => {
                        info!("Received STOP message");
                        break;
                    }
                }
            }
        });

        Ok(Self {
            path: path.to_path_buf(),
            folderid,
            pcloud,
            tx_file_close: tx,
            thread_file_close: Some(t),
        })
    }

    /// Complete any pending operations: joins the file_close thread
    pub async fn flush(&mut self) -> Result<()> {
        if self.tx_file_close.send(FileCloseMessage::Stop).is_ok() {
            self.thread_file_close
                .take()
                .ok_or_else(|| anyhow!("Thread is already closed!"))?
                .await?;
        }
        Ok(())
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

    async fn walk_directory(
        &self,
        tx: Sender<Self::Metadata>,
        _threads: usize,
        _custom_ignore_filename: &Path,
    ) -> Result<()> {
        // TODO: Implement _custom_ignore_filename logic

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

    async fn exists(&self, path: &Path) -> Result<bool> {
        let path = self.check_path(path)?;
        let _r = self.pcloud.stat(StatInput::Path(path)).await?;
        Ok(true)
    }

    async fn create(&self, path: &Path) -> Result<Box<dyn File>> {
        let path = self.check_path(path)?;

        let relative_path = path.strip_prefix(self.root())?;
        let filename = relative_path.file_name().unwrap().to_string_lossy().to_string();
        let folderid = match relative_path.parent() {
            None => self.folderid.clone(),
            Some(parent_dir) => {
                if parent_dir != Path::new("") {
                    get_folderid(&self.pcloud, parent_dir).await?
                } else {
                    self.folderid.clone()
                }
            }
        };

        let fd = self
            .pcloud
            .file_open(
                Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC,
                FileOpenPath::FolderAndName(folderid, filename),
            )
            .await?;

        let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone(), self.tx_file_close.clone());
        Ok(Box::new(f))
    }

    async fn open(&self, path: &Path) -> Result<Box<dyn File>> {
        let path = self.check_path(path)?;

        //let relative_path = v.strip_prefix(self.root())?;
        let fd = self
            .pcloud
            .file_open(Flags::empty(), FileOpenPath::Path(path.to_string_lossy().parse()?))
            .await?;

        let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone(), self.tx_file_close.clone());
        Ok(Box::new(f))
    }

    async fn create_dir_all(&self, path: &Path) -> Result<()> {
        let path = self.check_path(path)?;

        let mut folderid = self.folderid.clone();
        for cmp in path.components() {
            if let Component::Normal(p) = cmp {
                let input =
                    CreateFolderIfNotExistsInput::FolderAndName(folderid.clone(), p.to_string_lossy().to_string());
                let r = self.pcloud.createfolderifnotexists(&input).await?;
                folderid = r.metadata.folderid.unwrap();
            }
        }
        Ok(())
    }

    async fn remove_file(&self, path: &Path) -> Result<()> {
        let path = self.check_path(path)?;

        let input = DeleteFileInput::Path(path);
        self.pcloud.deletefile(input).await?;
        Ok(())
    }

    async fn remove_dir(&self, path: &Path) -> Result<()> {
        let path = self.check_path(path)?;

        let input = DeleteFolderInput::Path(path);
        self.pcloud.deletefolder(input).await?;
        Ok(())
    }

    async fn remove_dir_all(&self, path: &Path) -> Result<()> {
        let path = self.check_path(path)?;

        let input = DeleteFolderRecursiveInput::Path(path);
        self.pcloud.deletefolderrecursive(input).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pcloud_sdk::error::Error;
    use pcloud_sdk::methods::file::deletefile;
    use pcloud_sdk::methods::file::deletefile::DeleteFile;
    use pcloud_sdk::methods::fileops::file_open::FileOpen;
    use pcloud_sdk::methods::fileops::file_write::FileWrite;
    use pcloud_sdk::methods::fileops::{file_close, file_open, file_read, file_write};
    use pcloud_sdk::methods::folder::createfolderifnotexists::CreateFolderIfNotExists;
    use pcloud_sdk::methods::folder::deletefolder::DeleteFolder;
    use pcloud_sdk::methods::folder::deletefolderrecursive::DeleteFolderRecursive;
    use pcloud_sdk::methods::folder::listfolder::ListFolder;
    use pcloud_sdk::methods::folder::{createfolderifnotexists, deletefolder, deletefolderrecursive, listfolder};
    use pcloud_sdk::mocks::client::MockLocalClient;
    use pcloud_sdk::types::FileID;
    use pcloud_sdk::utils;

    use crate::file::CHUNK_SIZE;

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
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

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
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

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
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

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

        // Expectation for close
        client
            .expect_get::<()>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(())
            });

        // Create the filesystem
        let mut fs = FilesystemPCloud::new(Path::new("the/path"), client).await?;

        let filepath = Path::new("file");
        let content: Vec<u8> = b"Hello, world!".to_vec();

        // Create and write_all
        {
            let mut f = fs.create(&filepath).await?;
            f.write_all(&content).await?;
        }
        fs.flush().await?;
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
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

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

        let file_content = b"Hello, world!";
        // Expectation for read_to_end (first call)
        client
            .expect_get_bytes()
            .times(1)
            .withf(|endpoint: &str, params: &HashMap<_, _>| {
                endpoint == file_read::ENDPOINT && params.contains_key("count") && {
                    let count: usize = params.get("count").unwrap().parse().unwrap();
                    count == CHUNK_SIZE
                }
            })
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_read::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(file_content.to_vec())
            });

        // Expectation for read_to_end (last call, remaining buffer, returns 0)
        client
            .expect_get_bytes()
            .times(1)
            .withf(|endpoint: &str, params: &HashMap<_, _>| {
                endpoint == file_read::ENDPOINT && params.contains_key("count") && {
                    let count: usize = params.get("count").unwrap().parse().unwrap();
                    count == CHUNK_SIZE - file_content.len()
                }
            })
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_read::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(b"".to_vec())
            });

        // Expectation for close
        client
            .expect_get::<()>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(())
            });

        // Create the filesystem
        let mut fs = FilesystemPCloud::new(Path::new("the/path"), client).await?;

        let filepath = Path::new("file");

        // Open and read
        {
            let mut file = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(file_content.to_vec(), content_read);
        }
        fs.flush().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_in_subfolder() -> Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = Path::new("the/root/path");

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(2) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert!(params.contains_key("path"));
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

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
                assert_eq!(params.get("name"), Some(&"myfile.txt".to_string()));
                Ok(FileOpen {
                    fd: 42,
                    fileid: FileID(1234),
                })
            });

        // Expectation for close
        client
            .expect_get::<()>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(())
            });

        // Create the filesystem
        let mut fs = FilesystemPCloud::new(root_path, client).await?;
        {
            let filepath = Path::new("nested/nested2/myfile.txt");
            let r = fs.create(&filepath).await;
            assert!(r.is_ok());
        }
        fs.flush().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dir_all() -> Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = Path::new("the/root/path");

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&"the/root/path".to_string()));
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

                Ok(ListFolder {
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        // Expectation for create_dir_all
        client
            .expect_get::<CreateFolderIfNotExists>()
            .times(5) // One for each folder
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, createfolderifnotexists::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert!(params.contains_key("folderid"));
                assert!(params.contains_key("name"));

                Ok(CreateFolderIfNotExists {
                    created: Some(true),
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        let fs = FilesystemPCloud::new(root_path, client).await?;

        fs.create_dir_all(Path::new("nested/nested2")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_file() -> Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = Path::new("the/root/path");

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&root_path.to_string_lossy().parse()?));
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

                Ok(ListFolder {
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        // Expectation for remove_file
        client
            .expect_get::<DeleteFile>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, deletefile::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert!(params.contains_key("path"));

                Ok(DeleteFile {
                    id: "1234-0".to_string(),
                    metadata: Metadata {
                        fileid: Some(FileID(1234)),
                        ..Default::default()
                    },
                })
            });

        let fs = FilesystemPCloud::new(root_path, client).await?;
        fs.remove_file(Path::new("nested/nested2")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_folder() -> Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = Path::new("the/root/path");

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&root_path.to_string_lossy().parse()?));
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

                Ok(ListFolder {
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        // Expectation for remove_folder
        client
            .expect_get::<DeleteFolder>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, deletefolder::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert!(params.contains_key("path"));

                Ok(DeleteFolder {
                    id: "1234-0".to_string(),
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        let fs = FilesystemPCloud::new(root_path, client).await?;
        fs.remove_dir(Path::new("nested/nested2")).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_folder_recursive() -> Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = Path::new("the/root/path");

        // Expectation for FilesystemPCloud::new
        client
            .expect_get::<ListFolder>()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("path"), Some(&root_path.to_string_lossy().parse()?));
                assert_eq!(params.get("filtermeta"), Some(&"folderid,id".to_string()));

                Ok(ListFolder {
                    metadata: Metadata {
                        folderid: Some(FolderID(1234)),
                        ..Default::default()
                    },
                })
            });

        // Expectation for remove_folder
        client
            .expect_get::<DeleteFolderRecursive>()
            .times(1)
            .returning(move |endpoint, params: HashMap<_, _>| {
                assert_eq!(endpoint, deletefolderrecursive::ENDPOINT);
                assert_eq!(params.len(), 1);
                assert!(params.contains_key("path"));

                Ok(DeleteFolderRecursive {
                    deletedfiles: 10,
                    deletedfolders: 20,
                })
            });

        let fs = FilesystemPCloud::new(root_path, client).await?;
        fs.remove_dir_all(Path::new("nested/nested2")).await?;
        Ok(())
    }
}
