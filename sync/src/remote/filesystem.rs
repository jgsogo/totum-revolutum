use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;
use flume::Sender;
use tokio::time::Instant;
use tracing::{info, trace};

use pcloud_sdk::methods::folder::listfolder::GetListFolder;
use pcloud_sdk::methods::folder::ListFolderInput;
use pcloud_sdk::structures::Metadata;

use crate::actions;
use crate::diff::Filesystem;
use crate::local::{LocalFileMetadata, LocalMetadata};
use crate::remote::file_metadata::RemoteFileMetadata;
use crate::remote::RemoteMetadata;
use crate::storage::config;

pub struct FilesystemPCloud {
    pcloud: pcloud_sdk::client::HttpClient,
    tx: flume::Sender<RemoteMetadata>,
}

impl FilesystemPCloud {
    pub fn new(pcloud: pcloud_sdk::client::HttpClient, tx: flume::Sender<RemoteMetadata>) -> Self {
        Self { pcloud, tx }
    }

    pub async fn walk_directory(&self, _threads: usize, config: &config::Config) -> Result<()> {
        // FIXME: Here we can implement two different strategies. One of them is to iterate everything
        //  from the ROOT folder recursively, the other one is to list the files in each directory
        //  and use a thread pool to enter child directories and _recurse_.
        info!("Start remote visitor");
        let start = Instant::now();
        let mut list_folder_input = ListFolderInput::new_from_path(config.auth.remote_path.clone());
        list_folder_input.recursive = true;
        let filtermeta = vec!["name", "contents", "size", "hash"];
        let items = self
            .pcloud
            .listfolder_with_filtermeta(&list_folder_input, filtermeta)
            .await
            .unwrap();

        match &items.metadata.contents {
            Some(contents) => FilesystemPCloud::work_on_contents(Path::new(""), contents, self, 0)?,
            None => (),
        }
        info!("Finished remote visitor in {:?}", start.elapsed());

        Ok(())
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

    fn work_on_contents(base_path: &Path, contents: &[Metadata], diff: &FilesystemPCloud, depth: usize) -> Result<()> {
        for it in contents.iter() {
            let path = base_path.join(Path::new(it.common.name.as_ref().unwrap()));
            trace!("{}{}", format!("{}|-- ", " ".repeat(depth * 4)), path.display());
            diff.file_found(it.clone())?;
            match &it.contents {
                Some(contents) => FilesystemPCloud::work_on_contents(&path, contents, diff, depth + 1)?,
                None => (),
            }
        }
        Ok(())
    }
}

impl Filesystem for FilesystemPCloud {
    type Metadata = RemoteMetadata;

    fn tx(&self) -> &Sender<Self::Metadata> {
        &self.tx
    }
}

// #[async_trait]
// impl actions::Copy<LocalMetadata, RemoteMetadata> for FilesystemPCloud {
//     async fn copy(
//         &self,
//         _lhs: &LocalMetadata,
//         _rhs: Option<RemoteMetadata>,
//     ) -> Result<(&LocalMetadata, RemoteMetadata)> {
//         todo!()
//     }
// }
//
// #[async_trait]
// impl actions::Rename<RemoteMetadata> for FilesystemPCloud {
//     async fn rename(&self, _file: RemoteMetadata) -> Result<RemoteMetadata> {
//         todo!()
//     }
// }
