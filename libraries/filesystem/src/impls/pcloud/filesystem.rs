use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use camino::Utf8PathBuf;
use flume::Sender;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;
use tokio::time::Instant;
use tracing::{info, trace, warn};

use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::{Exists, GetCreateFolderIfNotExistsAll, GetFolderID};
use pcloud_sdk::methods::file::checksumfile::{ChecksumFile, GetChecksumFile};
use pcloud_sdk::methods::file::deletefile::GetDeleteFile;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::folder::deletefolder::GetDeleteFolder;
use pcloud_sdk::methods::folder::deletefolderrecursive::GetDeleteFolderRecursive;
use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::structures::Metadata;
use pcloud_sdk::types::errors::{InvalidFileError, InvalidFolderError};
use pcloud_sdk::types::{File as PCloudFile, FolderID, RemotePath};

use crate::filesystem::FilesystemOps;
use crate::ignore_filter::IgnoreFilterT;
use crate::impls::pcloud::file::RemoteFile;
use crate::{DirectoryPath, DirectoryPathBuf, Error, File, FileMetadata, FilePath, FilenameBuf, Filesystem, Result};

pub struct FilesystemPCloud<HttpClient: PCloudClient + Clone + Send + 'static> {
    // Root folder for this filesystem
    root_folderid: FolderID,
    root_path: Utf8PathBuf,

    // TODO: This shouldn't be an `Arc<HttpClient>`. It should be just `HttpClient`
    pcloud: Arc<HttpClient>,
}

impl<HttpClient: PCloudClient + Send + Clone + 'static> FilesystemPCloud<HttpClient> {
    pub async fn new(root_path: &RemotePath, pcloud: HttpClient) -> Result<Self> {
        let pcloud = Arc::new(pcloud);

        let root_folderid = pcloud
            .get_folderid(root_path)
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(Self {
            root_folderid,
            root_path: root_path.as_path().into(),
            pcloud,
        })
    }
}

#[async_trait]
impl<HttpClient: PCloudClient + Send + Clone + 'static> Filesystem for FilesystemPCloud<HttpClient> {
    async fn sync_all(mut self) -> Result<()> {
        Ok(())
    }

    async fn walk_directory(&self, tx: Sender<FileMetadata>, ignore_filter: IgnoreFilter) -> Result<()> {
        // FIXME: Here we can implement two different strategies. One of them is to iterate everything
        //  from the ROOT folder recursively, the other one is to list the files in each directory
        //  and use a thread pool to enter child directories and _recurse_.
        info!("Start remote visitor");
        let start = Instant::now();
        let filtermeta = vec!["name", "contents", "size", "hash", "isfolder"];

        let mut folders = vec![(self.root_folderid.clone(), DirectoryPathBuf::root(), 0)];
        while let Some((folderid, base_path, depth)) = folders.pop() {
            let list_folder_input = ListFolderInput::new(folderid.into());
            let items = self
                .pcloud
                .listfolder_with_filtermeta(list_folder_input, filtermeta.clone())
                .await
                .map_err(|e| Error::Other(e.to_string()))?;

            if let Some(contents) = items.metadata.contents {
                for it in contents.into_iter() {
                    match it {
                        Metadata::MetadataFile(m) => {
                            let filename = FilenameBuf::from_str(m.common.name.as_ref().unwrap())?;
                            let filepath = base_path.join_filename(&filename);
                            trace!("{}{}", format!("{}|-- ", " ".repeat(depth * 4)), filepath);

                            if ignore_filter.visit_file(&filepath) {
                                let remote_file = pcloud_sdk::types::File::FileID(m.fileid);
                                let checksumfile = self
                                    .pcloud
                                    .checksumfile(&remote_file)
                                    .await
                                    .map_err(|e| Error::Other(e.to_string()))?;

                                if checksumfile.sha256.is_none() || checksumfile.metadata.size.is_none() {
                                    warn!("sha256 and/or metadata.size are not available");
                                }

                                let data = FileMetadata {
                                    path: filepath,
                                    hash: checksumfile.sha256.unwrap(),
                                    size: checksumfile.metadata.size.unwrap(),
                                };
                                tx.send(data).map_err(|e| Error::Other(e.to_string()))?;
                            }
                        }
                        Metadata::MetadataFolder(m) => {
                            let path =
                                base_path.join(DirectoryPathBuf::from_str(m.common.name.as_ref().unwrap()).unwrap());
                            trace!("{}{}", format!("{}|-- ", " ".repeat(depth * 4)), path);

                            if ignore_filter.visit_directory(&path) {
                                folders.push((m.folderid, path, depth + 1));
                            }
                        }
                    }
                }
            }
        }

        info!("Finished remote visitor in {:?}", start.elapsed());

        Ok(())
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<FileMetadata> {
        let remote_path = {
            let abs_path = self.root_path.join(path);
            RemotePath::try_from(abs_path).map_err(|e| Error::Other(e.to_string()))?
        };
        let remote_file = pcloud_sdk::types::File::RemotePath(remote_path);

        // FIXME: Use checksumfile here to get the sha256 of the file
        let stat = self
            .pcloud
            .checksumfile_with_retry(
                &remote_file,
                Box::new(|r: pcloud_sdk::Result<ChecksumFile>| match r {
                    Ok(r) => {
                        if r.sha256.is_none() || r.metadata.size.is_none() {
                            Err(pcloud_sdk::Error::InputDataError(
                                "Hash and size are not available. Retry".to_string(),
                            ))
                        } else {
                            Ok(r)
                        }
                    }
                    Err(e) => Err(e),
                }),
            )
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(FileMetadata {
            path: path.to_filepath_buf(),
            hash: stat.sha256.unwrap(),
            size: stat.metadata.size.unwrap(),
        })
    }

    async fn exists(&self, path: &FilePath) -> Result<bool> {
        let dir = path.directory();
        let parent_folderid = match dir.parent() {
            None => self.root_folderid.clone(),
            Some(_) => {
                let abspath = self.root_path.join(dir);
                let rp = RemotePath::try_from(abspath).map_err(|e| Error::Other(e.to_string()))?;
                self.pcloud
                    .get_folderid(&rp)
                    .await
                    .map_err(|e| Error::Other(e.to_string()))?
            }
        };

        let r = self
            .pcloud
            .exists(parent_folderid, path.filename().as_str())
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(r.is_some())
    }

    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        let path = self.root_path.join(path);
        let path: PCloudFile = path
            .try_into()
            .map_err(|e: InvalidFileError| Error::Other(e.to_string()))?;

        //let relative_path = v.strip_prefix(self.root())?;
        let fd = self
            .pcloud
            .file_open(Flags::empty(), FileOpenPath::File(path))
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone());
        Ok((Box::new(f), None))
    }

    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        let dir = path.directory();
        // let rel_path = self.check_path(path)?;
        // let filename = rel_path.file_name().ok_or(Error::NotAFilepath)?;
        //
        // let rel_path = rel_path
        //     .parent()
        //     .map_or_else(|| None, |v| if v.as_str() == "" { None } else { Some(v) });

        let folder_id = match dir.parent() {
            None => self.root_folderid.clone(),
            Some(_) => {
                let abspath = self.root_path.join(dir);
                let rp = RemotePath::try_from(abspath).map_err(|e| Error::Other(e.to_string()))?;
                self.pcloud
                    .get_folderid(&rp)
                    .await
                    .map_err(|e| Error::Other(e.to_string()))?
            }
        };

        let fd = self
            .pcloud
            .file_open(
                Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC,
                FileOpenPath::FolderAndName(folder_id, path.filename().as_str().to_string()),
            )
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        let f = RemoteFile::<HttpClient>::new(fd, self.pcloud.clone());
        Ok((Box::new(f), None))
    }

    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.pcloud
            .createfolderifnotexists_all(&self.root_folderid, path)
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    async fn remove_file(&mut self, path: &FilePath) -> Result<()> {
        let path = self.root_path.join(path);
        let input_file = path
            .try_into()
            .map_err(|e: InvalidFileError| Error::Other(e.to_string()))?;
        self.pcloud
            .deletefile(input_file)
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()> {
        let path = self.root_path.join(path);
        let input_folder = path
            .try_into()
            .map_err(|e: InvalidFolderError| Error::Other(e.to_string()))?;
        self.pcloud
            .deletefolder(input_folder)
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        let path = self.root_path.join(path);
        let input_folder = path
            .try_into()
            .map_err(|e: InvalidFolderError| Error::Other(e.to_string()))?;
        self.pcloud
            .deletefolderrecursive(input_folder)
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    // TODO: Implement using some new endpoint
    // async fn internal_copy(
    //     &mut self,
    //     origin: &Utf8Path,
    //     target: &Utf8Path,
    //     force: bool,
    // ) -> Result<Option<Receiver<Result<()>>>> {
    //     todo!("impl internal_copy for PCloud. It probably needs some new endpoint")
    // }
    //
    // TODO: Implement using some new endpoint
    // async fn internal_move(
    //     &mut self,
    //     origin: &Utf8Path,
    //     target: &Utf8Path,
    //     force: bool,
    // ) -> Result<Option<Receiver<Result<()>>>> {
    //     todo!("impl internal_move for PCloud. It probably needs some new endpoint")
    // }
}

#[async_trait]
impl<HttpClient: PCloudClient + Send + Clone + 'static> FilesystemOps for FilesystemPCloud<HttpClient> {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use headers::HeaderMap;

    use pcloud_sdk::error::{Error, PCloudError};
    use pcloud_sdk::methods::file::deletefile;
    use pcloud_sdk::methods::file::deletefile::DeleteFile;
    use pcloud_sdk::methods::fileops::file_open::FileOpen;
    use pcloud_sdk::methods::fileops::file_write::FileWrite;
    use pcloud_sdk::methods::fileops::{file_close, file_open, file_read, file_write, FileDescriptor};
    use pcloud_sdk::methods::folder::createfolderifnotexists::CreateFolderIfNotExists;
    use pcloud_sdk::methods::folder::deletefolder::DeleteFolder;
    use pcloud_sdk::methods::folder::deletefolderrecursive::DeleteFolderRecursive;
    use pcloud_sdk::methods::folder::listfolder::ListFolder;
    use pcloud_sdk::methods::folder::{createfolderifnotexists, deletefolder, deletefolderrecursive, listfolder};
    use pcloud_sdk::mocks::client::MockLocalClient;
    use pcloud_sdk::structures::{MetadataFile, MetadataFolder};
    use pcloud_sdk::types::{FileID, Folder};
    use pcloud_sdk::utils;

    use crate::impls::pcloud::file::CHUNK_SIZE;
    use crate::wrappers::AsyncFileDropImpl;
    use crate::FilePathBuf;

    use super::*;

    #[tokio::test]
    async fn test_root() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();

        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                // let mut params = HashMap::new();
                // query.add_to_params(&mut params);

                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        let root = RemotePath::from_str("path:/the/path")?;
        let fs = FilesystemPCloud::new(&root, client).await?;
        assert_eq!("/the/path", fs.root_path.as_str()); // Root is always '/'
        assert_eq!(FolderID::new(1234), fs.root_folderid);
        Ok(())
    }

    #[tokio::test]
    async fn test_root_not_exists() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        client.expect_get::<ListFolder, _>().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Err(Error::PCloudError(PCloudError::DirectoryDoesNotExist))
            },
        );

        let root = RemotePath::from_str("path:/the/path")?;
        let r = client.get_folderid(&root).await;
        assert!(r.is_err());
        assert!(matches!(
            r.unwrap_err(),
            pcloud_sdk::Error::PCloudError(PCloudError::DirectoryDoesNotExist)
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_write_all() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();

        // Expectation for FilesystemPCloud::new
        client.expect_get::<ListFolder, _>().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            },
        );

        // Expectation for create
        client.expect_get::<FileOpen, _>().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, file_open::ENDPOINT);
                assert_eq!(params.len(), 3);
                let flags = (Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC).bits().to_string();
                assert_eq!(params.get("flags"), Some(&flags));
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"file".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(FileOpen {
                    fd: FileDescriptor::new(42),
                    fileid: FileID::new(1234),
                })
            },
        );

        // Expectation for write_all
        client.expect_post().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &FileDescriptor, posted_data: Vec<u8>| {
                assert_eq!(endpoint, file_write::ENDPOINT);
                assert_eq!(headers.len(), 1);
                assert_eq!(
                    headers.get("content-type").unwrap(),
                    "multipart/form-data; boundary=ea3bbcf87c101592"
                );
                assert_eq!(params, &FileDescriptor::new(42));

                let mut data = b"Hello, world!".to_vec();
                let bdata = utils::http::create_file_write(&mut data, "filename")?;
                assert_eq!(posted_data, bdata);
                Ok(FileWrite { bytes: 321 })
            },
        );

        // Expectation for close
        client.expect_get::<(), _>().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &FileDescriptor| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params, &FileDescriptor::new(42));
                Ok(())
            },
        );

        // Create the filesystem
        let root = RemotePath::from_str("path:/the/path")?;
        let fs = FilesystemPCloud::new(&root, client).await?;
        let mut fs = AsyncFileDropImpl::new_call_sync_all(fs);

        let rx = {
            let filepath = FilePathBuf::new(DirectoryPathBuf::root(), FilenameBuf::from_str("file").unwrap());
            let (mut f, rx) = fs.create(&filepath).await?;

            let content: Vec<u8> = b"Hello, world!".to_vec();
            f.write_all(&content).await?;
            rx
        };
        if let Some(rx) = rx {
            rx.await??;
        }

        fs.sync_all().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_open_read_all() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for open
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, file_open::ENDPOINT);
                assert_eq!(params.len(), 2);
                let flags = (Flags::empty()).bits().to_string();
                assert_eq!(params.get("flags"), Some(&flags));
                assert_eq!(params.get("path"), Some(&"/the/path/file".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(FileOpen {
                    fd: FileDescriptor::new(42),
                    fileid: FileID::new(1234),
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
            .returning(move |endpoint, params: &HashMap<_, _>| {
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
            .returning(move |endpoint, params: &HashMap<_, _>| {
                assert_eq!(endpoint, file_read::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert_eq!(params.get("fd"), Some(&"42".to_string()));
                Ok(b"".to_vec())
            });

        // Expectation for close
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &FileDescriptor| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params, &FileDescriptor::new(42));
                Ok(())
            });

        // Create the filesystem
        let root = RemotePath::from_str("path:/the/path")?;
        let fs = FilesystemPCloud::new(&root, client).await?;
        let fs = AsyncFileDropImpl::new_call_sync_all(fs);

        let filepath = FilePathBuf::new(DirectoryPathBuf::root(), FilenameBuf::from_str("file").unwrap());

        // Open and read
        let rx = {
            let (mut file, rx) = fs.open(&filepath).await?;
            let mut content_read = Vec::new();
            file.read_to_end(&mut content_read).await?;
            assert_eq!(file_content.to_vec(), content_read);
            rx.unwrap()
        };
        rx.await??;
        fs.sync_all().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_in_subfolder() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = RemotePath::from_str("path:/the/root/path")?;

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(2) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert!(params.contains_key("path"));
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for create
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, file_open::ENDPOINT);
                assert_eq!(params.len(), 3);
                let flags = (Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC).bits().to_string();
                assert_eq!(params.get("flags"), Some(&flags));
                assert_eq!(params.get("folderid"), Some(&"1234".to_string()));
                assert_eq!(params.get("name"), Some(&"myfile.txt".to_string()));
                assert_eq!(headers.len(), 0);
                Ok(FileOpen {
                    fd: FileDescriptor::new(42),
                    fileid: FileID::new(1234),
                })
            });

        // Expectation for close
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &FileDescriptor| {
                assert_eq!(endpoint, file_close::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params, &FileDescriptor::new(42));
                Ok(())
            });

        // Create the filesystem
        let fs = FilesystemPCloud::new(&root_path, client).await?;
        let mut fs = AsyncFileDropImpl::new_call_sync_all(fs);
        let rx = {
            let filepath = FilePathBuf::new(
                DirectoryPathBuf::from_str("nested/nested2").unwrap(),
                FilenameBuf::from_str("myfile.txt").unwrap(),
            );
            let (_, rx) = fs.create(&filepath).await?;
            rx.unwrap()
        };
        rx.await??;
        fs.sync_all().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dir_all() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = RemotePath::from_str("path:/the/root/path")?;

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/root/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for create_dir_all
        client
            .expect_get()
            .times(2) // One for each folder
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, createfolderifnotexists::ENDPOINT);
                assert_eq!(params.len(), 2);
                assert!(params.contains_key("folderid"));
                assert!(params.contains_key("name"));
                assert_eq!(headers.len(), 0);

                Ok(CreateFolderIfNotExists {
                    created: Some(true),
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        let mut fs = FilesystemPCloud::new(&root_path, client).await?;

        let dir = DirectoryPathBuf::from_str("nested/nested2").unwrap();
        fs.create_dir_all(&dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_file() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = RemotePath::from_str("path:/the/root/path")?;

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/root/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for remove_file
        client.expect_get().times(1).returning(
            move |endpoint, headers: HeaderMap, params: &pcloud_sdk::types::File| {
                assert_eq!(endpoint, deletefile::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(
                    params,
                    &pcloud_sdk::types::File::from_str("path:/the/root/path/nested/nested2/file").unwrap()
                );

                Ok(DeleteFile {
                    id: Some("1234-0".to_string()),
                    metadata: MetadataFile::default(FileID::new(1234)),
                })
            },
        );

        let mut fs = FilesystemPCloud::new(&root_path, client).await?;
        let dir = DirectoryPathBuf::from_str("nested/nested2").unwrap();
        fs.remove_file(&dir.join_filename(FilenameBuf::from_str("file").unwrap()))
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_folder() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = RemotePath::from_str("path:/the/root/path")?;

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/root/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for remove_folder
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &Folder| {
                assert_eq!(endpoint, deletefolder::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params, &Folder::from_str("path:/the/root/path/nested/nested2").unwrap());

                Ok(DeleteFolder {
                    id: "1234-0".to_string(),
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        let mut fs = FilesystemPCloud::new(&root_path, client).await?;
        let dir = DirectoryPathBuf::from_str("nested/nested2").unwrap();
        fs.remove_dir(&dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_folder_recursive() -> anyhow::Result<()> {
        let mut client = MockLocalClient::new();
        let root_path = RemotePath::from_str("path:/the/root/path")?;

        // Expectation for FilesystemPCloud::new
        client
            .expect_get()
            .times(1) // One on filesystem::new, another to check folder for file being created
            .returning(move |endpoint, headers: HeaderMap, params: &HashMap<_, _>| {
                assert_eq!(endpoint, listfolder::ENDPOINT);
                assert_eq!(params.len(), 3);
                assert_eq!(params.get("path").unwrap(), "/the/root/path");
                assert_eq!(params.get("filtermeta").unwrap(), "folderid,fileid");
                assert_eq!(params.get("recursive").unwrap(), "0");
                assert_eq!(headers.len(), 0);

                Ok(ListFolder {
                    metadata: MetadataFolder::default(FolderID::new(1234)),
                })
            });

        // Expectation for remove_folder
        client
            .expect_get()
            .times(1)
            .returning(move |endpoint, headers: HeaderMap, params: &Folder| {
                assert_eq!(endpoint, deletefolderrecursive::ENDPOINT);
                assert_eq!(headers.len(), 0);
                assert_eq!(params, &Folder::from_str("path:/the/root/path/nested/nested2").unwrap());

                Ok(DeleteFolderRecursive {
                    deletedfiles: 10,
                    deletedfolders: 20,
                })
            });

        let mut fs = FilesystemPCloud::new(&root_path, client).await?;
        let dir = DirectoryPathBuf::from_str("nested/nested2").unwrap();
        fs.remove_dir_all(&dir).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_walk_directory() -> anyhow::Result<()> {
        let client = pcloud_sdk::mocks::filesystem::filesystem_mocked();
        let root_path = RemotePath::from_str("path:/mocked_filesystem").unwrap();
        let fs = FilesystemPCloud::new(&root_path, client).await?;

        // Root directory, empty filters
        {
            let ignore_filter = IgnoreFilter::empty("");

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir1/other/file.png",
                    "dir1/other/file.rs",
                    "dir1/subdir/file.png",
                    "dir1/subdir/file.rs",
                    "dir2/file.png",
                    "dir2/file.rs",
                    "file.png",
                    "file.rs",
                    "folder1/dir1/file.png",
                    "folder1/dir1/file.rs",
                    "folder1/file.png",
                    "folder1/file.rs"
                ]
            );
        }

        // Ignore all '*.rs' files
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["*.rs"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir1/other/file.png",
                    "dir1/subdir/file.png",
                    "dir2/file.png",
                    "file.png",
                    "folder1/dir1/file.png",
                    "folder1/file.png"
                ]
            );
        }

        // Ignore everything inside "dir1/" directory (also if nested)
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["dir1/"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir2/file.png",
                    "dir2/file.rs",
                    "file.png",
                    "file.rs",
                    "folder1/file.png",
                    "folder1/file.rs"
                ]
            );
        }

        // Ignore everything inside "dir1/" directory (only if root), and ignore all `.rs` files
        // inside folder1/ path
        {
            let mut ignore_filter = IgnoreFilter::empty("");
            ignore_filter.add_globs(&["/dir1/", "folder1/**/*.rs"], None).unwrap();

            let (tx, rx) = flume::bounded(100);
            fs.walk_directory(tx, ignore_filter).await.unwrap();

            let all_files = rx.try_iter().collect::<Vec<_>>();
            let mut all_files_str: Vec<&str> = all_files.iter().map(|p| p.path().as_str()).collect();
            all_files_str.sort();

            assert_eq!(
                all_files_str,
                vec![
                    "dir2/file.png",
                    "dir2/file.rs",
                    "file.png",
                    "file.rs",
                    "folder1/dir1/file.png",
                    "folder1/file.png"
                ]
            );
        }

        Ok(())
    }
}
