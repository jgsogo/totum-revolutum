use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use flume::Sender;
use ignore_files::IgnoreFilter;
use tokio::sync::oneshot::Receiver;

use crate::{DirectoryPath, Error, File, FileMetadata, FilePath, Filesystem, FilesystemOps, Result};

pub(crate) const SUCCESS: &str = "success";

/// A [`Filesystem`] implementation that only records the methods called and their arguments. All
/// the methods will return an error unless the given path is equal to `"success"` (and it's possible
/// to return something meaningful).
pub struct FilesystemMock {
    id: String,
    pub called: Arc<RwLock<Vec<(String, Vec<String>)>>>,
}

impl FilesystemMock {
    pub fn new(id: &str, data: Arc<RwLock<Vec<(String, Vec<String>)>>>) -> Self {
        Self {
            id: id.to_string(),
            called: data,
        }
    }

    pub fn methods(&self) -> Vec<String> {
        self.called
            .read()
            .unwrap()
            .iter()
            .map(|v| v.0.clone())
            .collect::<Vec<_>>()
    }
}

#[async_trait]
impl Filesystem for FilesystemMock {
    async fn sync_all(self) -> Result<()> {
        self.called.write().unwrap().push(("sync_all".to_string(), vec![]));
        Err(Error::Other(format!(
            "FilesystemMock('{}') doesn't execute actual work",
            self.id
        )))
    }

    async fn walk_directory(&self, _tx: Sender<Box<dyn FileMetadata>>, _ignore_filter: IgnoreFilter) -> Result<()> {
        self.called
            .write()
            .unwrap()
            .push(("walk_directory".to_string(), vec![]));
        Err(Error::Other(format!(
            "FilesystemMock('{}') doesn't execute actual work",
            self.id
        )))
    }

    async fn get_metadata(&self, path: &FilePath) -> Result<Box<dyn FileMetadata>> {
        self.called
            .write()
            .unwrap()
            .push(("get_metadata".to_string(), vec![path.to_string()]));
        Err(Error::Other(format!(
            "FilesystemMock('{}') doesn't execute actual work",
            self.id
        )))
    }

    async fn exists(&self, path: &FilePath) -> Result<bool> {
        self.called
            .write()
            .unwrap()
            .push(("exists".to_string(), vec![path.to_string()]));
        if path.as_str() == SUCCESS {
            Ok(true)
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn open(&self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.called
            .write()
            .unwrap()
            .push(("open".to_string(), vec![path.to_string()]));
        Err(Error::Other(format!(
            "FilesystemMock('{}') doesn't execute actual work",
            self.id
        )))
    }

    async fn create(&mut self, path: &FilePath) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        self.called
            .write()
            .unwrap()
            .push(("create".to_string(), vec![path.to_string()]));
        Err(Error::Other(format!(
            "FilesystemMock('{}') doesn't execute actual work",
            self.id
        )))
    }

    async fn create_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.called
            .write()
            .unwrap()
            .push(("create_dir_all".to_string(), vec![path.to_string()]));
        if path.as_str() == SUCCESS {
            Ok(())
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn remove_file(&mut self, path: &FilePath) -> Result<()> {
        self.called
            .write()
            .unwrap()
            .push(("remove_file".to_string(), vec![path.to_string()]));
        if path.as_str() == SUCCESS {
            Ok(())
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn remove_dir(&mut self, path: &DirectoryPath) -> Result<()> {
        self.called
            .write()
            .unwrap()
            .push(("remove_dir".to_string(), vec![path.to_string()]));
        if path.as_str() == SUCCESS {
            Ok(())
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn remove_dir_all(&mut self, path: &DirectoryPath) -> Result<()> {
        self.called
            .write()
            .unwrap()
            .push(("remove_dir_all".to_string(), vec![path.to_string()]));
        if path.as_str() == SUCCESS {
            Ok(())
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn internal_copy(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        self.called.write().unwrap().push((
            "internal_copy".to_string(),
            vec![origin.to_string(), target.to_string(), force.to_string()],
        ));
        if target.as_str() == SUCCESS {
            Ok(None)
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }

    async fn internal_move(
        &mut self,
        origin: &FilePath,
        target: &FilePath,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        self.called.write().unwrap().push((
            "internal_move".to_string(),
            vec![origin.to_string(), target.to_string(), force.to_string()],
        ));
        if target.as_str() == SUCCESS {
            Ok(None)
        } else {
            Err(Error::Other(format!(
                "FilesystemMock('{}') doesn't execute actual work",
                self.id
            )))
        }
    }
}

impl FilesystemOps for FilesystemMock {}
