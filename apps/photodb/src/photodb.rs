use std::str::FromStr;

use anyhow::{anyhow, Result};
use camino::Utf8Path;
use diesel::{RunQueryDsl, SelectableHelper};
use log::error;
use tracing::{debug, info, trace};

use filesystem::impls::composites::indexed::diesel_indexed::DatabaseImpl;
use filesystem::impls::composites::FilesystemIndexed;
use filesystem::impls::{FilesystemLocal, FilesystemPCloud};
use filesystem::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf, Filesystem};
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::GetFileID;
use pcloud_sdk::types::FileID;
use pcloud_sdk::types::RemotePath;

use crate::metadata::{CollectMetadataFrom, MetadataCollector};

use super::database::models;
use super::database::Database;
use super::utils::sha256_string_from_file;
use super::AppDirs;

// Path inside the remote folder to locate files using sha256 filename
const SHA256_BASE_PATH: &str = "_sha256";

#[allow(dead_code)]
pub struct PhotoDB<'a, T: Database, TPCloudClient: PCloudClient + Clone + Send + 'static> {
    /// The PCloud client.
    pcloud: TPCloudClient,

    /// The root folder in the FilesystemPCloud storage. We need it when using `pcloud` directly to
    /// interact with the files.
    /// FIXME: Maybe wrap `pcloud` client together with this `root` so we don't need to worry about it
    root_folder: RemotePath,

    /// An instance of the [`Database`]. This is the same database that the [`Self::storage`] uses to
    /// index the files.
    db: T,

    /// Directories used by this instance of the PhotoDB application
    app_dir: &'a mut AppDirs,

    /// Indexed storage. Everything saved here will be mirrored to the DB (using the default
    /// [`filesystem_db_models::File`] and [`filesystem_db_models::Directory`] models).
    storage: FilesystemIndexed<DatabaseImpl, FilesystemPCloud<TPCloudClient>>,
}

impl<'a, T: Database, TPCloudClient: PCloudClient + Clone + Send + 'static> PhotoDB<'a, T, TPCloudClient> {
    /// Creates a new instance of [`PhotoDB`] using the given arguments:
    /// * `db` is the database instance, behind the [`Database`] trait.
    /// * `db_path` Path inside PCloud to mount the remote storage filesystem.
    /// * `app_dir`: An [`AppDirs`] reference with local paths to application directories
    /// * `pcloud`: The [`PCloudClient`] to use for the remote storage filesystem (and other
    ///   operations that require access to PCloud itself
    pub async fn new(db: T, db_path: &RemotePath, app_dir: &'a mut AppDirs, pcloud: TPCloudClient) -> Result<Self> {
        info!(
            "New photodb application using local directory '{}' and remote pcloud storage",
            app_dir
        );
        let database_impl = DatabaseImpl::new_from_connection(db.get_pool());
        let remote_storage = FilesystemPCloud::new(db_path, pcloud.clone()).await?;
        let storage = FilesystemIndexed::new(database_impl, remote_storage);
        // TODO: Spawn "self.sync" in parallel - self.sync().await?;
        Ok(Self {
            pcloud,
            root_folder: db_path.clone(),
            db,
            app_dir,
            storage,
        })
    }

    pub async fn add(&mut self, photo_filepath: impl AsRef<Utf8Path>) -> Result<()> {
        debug!("Add photo from path '{}'", photo_filepath.as_ref());

        let mut metadata_collector = MetadataCollector::default();
        metadata_collector.collect_from(photo_filepath.as_ref());
        metadata_collector.collect_from(std::fs::metadata(photo_filepath.as_ref())?);

        debug!("Execute [Self::add_from_file] to process and upload the file");
        self.add_from_local_file(photo_filepath, metadata_collector).await?;

        Ok(())
    }

    /// Iterates all the files in the DB and the files in the remote storage performing
    /// a [`filesystem::diff::impls::mirror`] operation. After it finishes it will iterate all
    /// the [`models::File`] that are not a [`models::PhotoFile`] and process the
    /// files to create the corresponding entries (as a result of this process, files might be
    /// moved to a different location).
    ///
    /// Note that, for the files that have been removed from the storage, the `mirror` operation
    /// will remove the [`models::File`] and ON CASCADE the corresponding [`models::PhotoFile`]
    /// will be removed.
    pub async fn sync(&mut self) -> Result<()> {
        let ignore_filter = self.app_dir.ignore_filters().await?;
        self.storage.initial_sync(ignore_filter).await.map_err(|e| anyhow!(e))?;

        let non_identified = self.db.get_orphan_files()?;
        debug!(
            "Found {} files not processed (they appear as models::File, but no models::PhotoFile)",
            non_identified.len()
        );

        if non_identified.is_empty() {
            return Ok(());
        }

        // Iterate all the orphan files, process and upload them to the right paths
        let mut conn = self.db.get_connection()?;

        // TODO: Parallelize here
        for it in non_identified {
            let origin_remote_path = it.full_path(&mut conn)?;
            trace!(" - {}", origin_remote_path);

            let mut metadata_collector = MetadataCollector::default();
            metadata_collector.collect_from(origin_remote_path.as_utf8_path());

            debug!("Download remote file to the local storage");
            let (local_tmp_filepath, rx) = self.app_dir.copy_to_tmp(&self.storage, &origin_remote_path).await?;
            if let Some(rx) = rx {
                rx.await??;
            }

            debug!("Execute [Self::add_from_local_file] to process and upload the file");
            let photo_path = self
                .add_from_local_file(&local_tmp_filepath, metadata_collector)
                .await?;

            debug!("Remove the orphan file");
            if photo_path != origin_remote_path {
                self.storage.remove_file(&origin_remote_path).await?;
            }
        }

        Ok(())
    }

    /// Iterates all the files in the storage using [`Filesystem::walk_directory`] and prints their
    /// full path. Note that this method iterates the rows in [`models::File`], execute [`Self::sync`]
    /// to ensure that all of them are also [`models::PhotoFile`] rows.
    pub async fn list(&self) -> Result<()> {
        let (tx, rx) = flume::bounded::<FileMetadata>(32);

        tokio::spawn(async move {
            let mut count_files = 0;
            while let Ok(r) = rx.recv() {
                println!("{}", r.path());
                count_files += 1;
            }
            println!("{} files total", count_files)
        });

        if let Err(e) = self
            .storage
            .walk_directory(tx, self.app_dir.ignore_filters().await?)
            .await
        {
            error!("Error iterating storage files: {e}");
        }

        Ok(())
    }

    /// Adds a file from the filesystem to the storage and then creates the corresponding
    /// [`models::PhotoFile`] entry in the database. Returns the location of the final file in the
    /// storage.
    ///
    /// Note.- The path here might no longer be the original path, so no metadata can be collected
    /// related to the path or filesystem.
    async fn add_from_local_file<P: AsRef<Utf8Path>>(
        &mut self,
        filepath: P,
        mut metadata_collector: MetadataCollector,
    ) -> Result<FilePathBuf> {
        // Retrieve EXIF data (before transformation, just in case)
        let exif = crate::exif::get_exif_data(&filepath).ok();
        if let Some(exif) = exif {
            metadata_collector.collect_from(&exif);
        }

        // Execute all the processing
        let (photo, format) = crate::image::prepare_image_file(&filepath)?;

        // Compute the target directory path and filename
        let diretory_path = match get_directory_from_metadata(&metadata_collector) {
            None => get_directory_using_sha256(photo.path())?,
            Some(d) => d,
        };
        let filename = FilenameBuf::from_str(filepath.as_ref().file_name().unwrap())?;
        let filepath = FilePathBuf::new(diretory_path, filename);

        // Do the upload (creates the corresponding [`models::File`] entry)
        info!("Upload to '{}'", filepath);
        let rx = {
            let photo_as_filepath = {
                let directory = photo.path().parent().unwrap();
                let directory = DirectoryPathBuf::try_from(directory.strip_prefix("/")?)?;

                let filename = photo.path().file_name().unwrap();
                let filename = FilenameBuf::from_str(filename)?;

                FilePathBuf::new(directory, filename)
            };
            self.storage.create_dir_all(filepath.directory()).await?;
            filesystem::actions::copy_file(
                &FilesystemLocal::local_hd(),
                &mut self.storage,
                &photo_as_filepath,
                &filepath,
                false,
            )
            .await?
        };

        debug!("Now create the PhotoFile entry for {}", filepath);
        let mut conn = self.db.get_connection()?;
        let format_ = models::Format::find(&format, &mut conn)?;
        rx.unwrap().await??; // Wait for the upload and DB entry creation

        // FIXME: Here we need an absolute path to satisfy pcloud's RemotePath... we need to
        // FIXME: consolidate filesystem with pcloud_sdk (maybe the other way around) so they
        // FIXME: both uses a relative path (whatever is root will always be prepended)
        let remote_abs_filepath = self.root_folder.join(&filepath)?;
        debug!("Get fileid for {}", remote_abs_filepath);
        let fileid_: FileID = self.pcloud.get_fileid(&remote_abs_filepath).await?;

        debug!("Get the models::File row for {}", &filepath);
        let file_ = self.db.get_file(&filepath)?;
        info!("Metadata: {:?}", metadata);
        let new_photo_file = models::PhotoFile::new_from(&file_, &fileid_, &format_, true, metadata_collector.into());

        use crate::database::schema::photo_files::dsl::*;
        let photo = diesel::insert_into(photo_files)
            .values(&new_photo_file)
            .returning(models::PhotoFile::as_returning())
            .get_result(&mut conn)?;

        debug!("Photo inserted into database: {}", photo.fileid);

        Ok(filepath)
    }
}

fn get_directory_using_sha256<P: AsRef<Utf8Path>>(photo: P) -> Result<DirectoryPathBuf> {
    let sha256 = sha256_string_from_file(photo)?;
    let (c1, rest) = sha256.split_at(2);
    let (c2, rest) = rest.split_at(2);
    let (c3, _rest) = rest.split_at(2);

    let path = Utf8Path::new(SHA256_BASE_PATH).join(c1).join(c2).join(c3);
    let path = DirectoryPathBuf::from_str(path.as_str())?;
    Ok(path)
}

fn get_directory_from_metadata(metadata_collector: &MetadataCollector) -> Option<DirectoryPathBuf> {
    match metadata_collector.get_candidate_date() {
        None => None,
        Some(date) => {
            let directory =
                DirectoryPathBuf::from_str(&date.to_string()).expect("Failed to create DirectoryPath from date");
            Some(directory)
        }
    }
}
