use anyhow::Result;
use async_trait::async_trait;
use camino::Utf8Path;

use log::error;
use tracing::{debug, info};

use filesystem::impls::FilesystemLocalSync;
use filesystem::{DirectoryPath, FilePath, Filesystem};
use pcloud_sdk::client::PCloudClient;

use crate::database::Database;
use crate::{CollectMetadataFrom, MetadataCollector, PhotoDB};

#[async_trait]
pub trait PhotoDBAdd<FS: Filesystem + Clone + 'static> {
    /// Adds the file at the give [`FilePath`] from the given `filesystem`.
    async fn add_from_file(
        &mut self,
        filesystem: &FS,
        filepath: &FilePath,
        metadata_collector: MetadataCollector,
    ) -> Result<()>;

    /// Adds all the files in the given [`DirectoryPath`] from the given `filesystem`.
    ///
    /// FIXME: Honor `directory` and `recursive` arguments. Alternative, probably we want to pass
    /// FIXME: here an IgnoreFilter and use it.
    async fn add_from_directory(
        &mut self,
        filesystem: FS,
        _directory: &DirectoryPath,
        _recursive: bool,
        metadata_collector: MetadataCollector,
    ) -> Result<()> {
        let (tx, rx) = flume::bounded(10);

        let fs = filesystem.clone();
        tokio::spawn(async move {
            let ignore_filter = fs.create_ignore_filter().await;
            if let Err(e) = fs.walk_directory(tx, ignore_filter).await {
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
impl<T: Database + Send, TPCloudClient: PCloudClient + Clone + Send + 'static> PhotoDBAdd<FilesystemLocalSync>
    for PhotoDB<'_, T, TPCloudClient>
{
    async fn add_from_file(
        &mut self,
        filesystem: &FilesystemLocalSync,
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
