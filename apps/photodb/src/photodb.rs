use std::str::FromStr;

use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use chrono::{Datelike, Timelike};
use diesel::{RunQueryDsl, SelectableHelper};
use exif::Exif;
use log::error;
use tracing::{debug, info, trace};

use filesystem::impls::composites::indexed::diesel_indexed::DatabaseImpl;
use filesystem::impls::composites::FilesystemIndexed;
use filesystem::impls::{FilesystemLocal, FilesystemLocalTemp, FilesystemPCloud};
use filesystem::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf, Filesystem, FilesystemOps};
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::GetFileID;
use pcloud_sdk::types::FileID;
use pcloud_sdk::types::RemotePath;

use crate::database::models::Formats;

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
    app_dir: &'a AppDirs,

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
    pub async fn new(db: T, db_path: &RemotePath, app_dir: &'a AppDirs, pcloud: TPCloudClient) -> Result<Self> {
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

    /// Takes a file from a local path (`original`), copy and process it using all the filters
    /// configured in PhotoDB application. Afterward uploads it to the remote storage and returns
    /// the remote location.
    async fn process_and_upload(
        &mut self,
        original: impl AsRef<Utf8Path>,
    ) -> Result<(
        FilePathBuf,
        Formats,
        Option<Exif>,
        tokio::sync::oneshot::Receiver<filesystem::Result<()>>,
    )> {
        info!("Processing file '{}'", original.as_ref());

        // Retrieve EXIF data (use original file just in case we lose/modify something in the transformations
        let exif = crate::exif::get_exif_data(original.as_ref()).ok();

        // Execute all the processing
        let (photo, format) = crate::image::prepare_image_file(original.as_ref())?;

        // Calculate the filepath
        let extension = format
            .as_extension()
            .unwrap_or(original.as_ref().extension().unwrap_or("").to_string());
        let filepath_from_sha256 = || -> Result<FilePathBuf> {
            let sha256 = sha256_string_from_file(photo.path())?;
            let (c1, rest) = sha256.split_at(4);
            let (c2, rest) = rest.split_at(4);

            let path = Utf8Path::new(SHA256_BASE_PATH).join(c1).join(c2);
            let path = DirectoryPathBuf::from_str(path.as_str())?;
            Ok(path.join_filename(FilenameBuf::from_str(&format!("{}.{}", rest, extension)).unwrap()))
        };
        let filepath = match &exif {
            Some(exif) => match crate::exif::get_creation_date(exif) {
                Ok(date) => {
                    let directory = DirectoryPathBuf::from_str(&format!(
                        "{y:04}/{m:02}/{d:02}",
                        y = date.year(),
                        m = date.month(),
                        d = date.day(),
                    ))?;
                    let filename = FilenameBuf::from_str(&format!(
                        "{y:04}-{m:02}-{d:02}-{H:02}{M:02}{S:02}.{ext}",
                        y = date.year(),
                        m = date.month(),
                        d = date.day(),
                        H = date.hour(),
                        M = date.minute(),
                        S = date.second(),
                        ext = extension
                    ))?;
                    directory.join_filename(filename)
                }
                Err(_) => filepath_from_sha256()?,
            },
            None => filepath_from_sha256()?,
        };

        // Do the upload
        info!("Upload to '{}'", filepath);
        let rx = {
            let photo_as_filepath = {
                let directory = Utf8PathBuf::from_path_buf(photo.path().parent().unwrap().to_path_buf()).unwrap();
                let directory = DirectoryPathBuf::try_from(directory.strip_prefix("/")?)?;

                let filename = photo.path().file_name().unwrap().to_str().unwrap();
                let filename = FilenameBuf::from_str(filename)?;

                FilePathBuf::new(directory, filename)
            };
            self.storage.create_dir_all(filepath.directory()).await?;
            let local_hd = FilesystemLocal::local_hd();
            filesystem::actions::copy_file(&local_hd, &mut self.storage, &photo_as_filepath, &filepath, false).await?
        };

        Ok((filepath, format, exif, rx.unwrap()))
    }

    pub async fn add(&mut self, photo_filepath: impl AsRef<Utf8Path>) -> Result<()> {
        debug!("Add photo from path '{}'", photo_filepath.as_ref());

        let (remote_filepath, formats_, _exif, rx) = self.process_and_upload(&photo_filepath).await?;
        // TODO: Decide what to do with EXIF data. Make sure the processed files contain the
        // TODO: same exif, we don't want to lose it.

        debug!("Now create the PhotoFile entry for {}", remote_filepath);
        let mut conn = self.db.get_connection()?;
        let format_ = models::Format::find(formats_, &mut conn)?;
        rx.await??; // Wait for the upload and DB entry creation

        // FIXME: Here we need an absolute path to satisfy pcloud's RemotePath... we need to
        // FIXME: consolidate filesystem with pcloud_sdk (maybe the other way around) so they
        // FIXME: both uses a relative path (whatever is root will always be prepended)
        let remote_abs_filepath = self.root_folder.join(&remote_filepath)?;
        // let remote_path = RemotePath::try_from(remote_abs_filepath)?;
        debug!("Get fileid for {}", remote_abs_filepath);
        let fileid_: FileID = self.pcloud.get_fileid(&remote_abs_filepath).await?;

        debug!("Get the models::File row for {}", &remote_filepath);
        let file_ = self.db.get_file(&remote_filepath)?;

        let new_photo_file = models::PhotoFile::new_from(&file_, &fileid_, &format_, true);

        use crate::database::schema::photo_files::dsl::*;
        let photo = diesel::insert_into(photo_files)
            .values(&new_photo_file)
            .returning(models::PhotoFile::as_returning())
            .get_result(&mut conn)?;

        debug!("Photo inserted into database: {}", photo.fileid);

        Ok(())
    }

    /// Iterates all the files in the DB and the files in the remote storage performing
    /// a [`filesystem::diff::impls::mirror`] operation. After it finishes it will iterate all
    /// the [`models::File`] that are not a [`models::PhotoFile`] and process the
    /// files to create the corresponding entries.
    ///
    /// Note that, for the files that have been removed from the storage, the `mirror` operation
    /// will remove the [`models::File`] and ON CASCADE the corresponding [`models::PhotoFile`]
    /// will be removed.
    pub async fn sync(&mut self) -> Result<()> {
        let ignore_filter = self.app_dir.ignore_filters().await?;
        self.storage.initial_sync(ignore_filter).await.map_err(|e| anyhow!(e))?;

        let non_identified = self.db.get_orphan_files()?;
        debug!("Found {} files not processed", non_identified.len());

        if non_identified.is_empty() {
            return Ok(());
        }

        // Iterate all the orphan files, process and upload them to the right paths
        let mut conn = self.db.get_connection()?;
        let mut tmp_filesystem = FilesystemLocalTemp::default();

        // TODO: Parallelize here
        for it in non_identified {
            let origin_remote_path = it.full_path(&mut conn)?;
            trace!(" - {}", origin_remote_path);

            debug!("Download remote file to the local storage");
            let local_filepath = {
                let tmp_path = tmp_filesystem.temp_filename(None, Some(origin_remote_path.filename().as_str()));
                tmp_filesystem
                    .copy_from(&tmp_path, &self.storage, &origin_remote_path, true)
                    .await?;
                tmp_filesystem.resolve_filepath(tmp_path)
            };

            debug!("Execute regular [Self::add] to process and upload the file");
            self.add(local_filepath).await?;

            debug!("Remove the orphan file");
            self.storage.remove_file(&origin_remote_path).await?;
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
}
