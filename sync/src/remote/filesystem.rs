use std::path::{Path, PathBuf};

use anyhow::Result;
use async_trait::async_trait;
use flume::Sender;
use tokio::time::Instant;
use tracing::{info, trace};

use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::structures::Metadata;

use crate::diff::{File, Filesystem};
use crate::local::LocalFileMetadata;
use crate::remote::RemoteMetadata;

pub struct FilesystemPCloud {
    path: PathBuf,
    pcloud: pcloud_sdk::client::HttpClient,
}

impl FilesystemPCloud {
    pub fn new(path: &Path, pcloud: pcloud_sdk::client::HttpClient) -> Self {
        // TODO: Probably check it doesn't contain any remaining `../`
        assert!(path.is_absolute());
        Self {
            path: path.to_path_buf(),
            pcloud,
        }
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
                FilesystemPCloud::work_on_contents(tx.clone(), &path, it.contents.as_ref().unwrap(), depth + 1)?
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Filesystem for FilesystemPCloud {
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
            Some(contents) => FilesystemPCloud::work_on_contents(tx, Path::new(""), contents, 0)?,
            None => (),
        }
        info!("Finished remote visitor in {:?}", start.elapsed());

        Ok(())
    }

    fn open(&self, _path: &Path) -> Result<Box<dyn File>> {
        todo!()
    }
}
