use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8Path;
use ignore_files::IgnoreFilter;
use log::error;
use tracing::{debug, info};

use filesystem::impls::FilesystemLocal;
use filesystem::{DirectoryPath, FilePath, Filesystem};
use pcloud_sdk::client::PCloudClient;

use crate::database::Database;
use crate::{CollectMetadataFrom, MetadataCollector, PhotoDB};

#[async_trait]
pub trait PhotoDBAdd<FS: Filesystem + Clone + 'static> {
    async fn add_from_file(
        &mut self,
        filesystem: &FS,
        filepath: &FilePath,
        metadata_collector: MetadataCollector,
    ) -> Result<()>;

    async fn add_from_directory(
        &mut self,
        filesystem: FS,
        _directory: &DirectoryPath, // TODO: start in this _directory
        _recursive: bool,           // TODO: Honor this argument
        metadata_collector: MetadataCollector,
    ) -> Result<()> {
        let (tx, rx) = flume::bounded(10);

        let fs = filesystem.clone();
        tokio::spawn(async move {
            if let Err(e) = fs.walk_directory(tx, IgnoreFilter::empty("")).await {
                error!("Error executing 'Filesystem::walk_directory': {e}");
            }
        });

        while let Ok(v) = rx.recv() {
            self.add_from_file(&filesystem, v.path(), metadata_collector.clone())
                .await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<'a, T: Database + Send, TPCloudClient: PCloudClient + Clone + Send + 'static> PhotoDBAdd<FilesystemLocal>
    for PhotoDB<'a, T, TPCloudClient>
{
    async fn add_from_file(
        &mut self,
        filesystem: &FilesystemLocal,
        filepath: &FilePath,
        mut metadata_collector: MetadataCollector,
    ) -> Result<()> {
        let photo_filepath_buf = filesystem.resolve_filepath(filepath);
        let photo_filepath: &Utf8Path = &photo_filepath_buf;

        info!("Add photo from path '{}'", photo_filepath);

        metadata_collector.collect_from(photo_filepath);
        metadata_collector.collect_from(std::fs::metadata(photo_filepath.as_std_path())?);

        debug!("Execute [Self::add_from_local_file] to process and upload the file");
        self.add_from_local_file(photo_filepath, metadata_collector).await?;

        Ok(())
    }
}
