use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use camino::Utf8Path;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;
use tracing::{debug, info, trace};

use filesystem::impls::composites::indexed::diesel_indexed::DatabaseImpl;
use filesystem::impls::composites::FilesystemIndexed;
use filesystem::impls::{FilesystemLocalTemp, FilesystemPCloud};
use filesystem::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf, Filesystem, FilesystemOps};
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::GetFileID;
use pcloud_sdk::types::FileID;
use pcloud_sdk::types::RemotePath;

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
    ) -> Result<(FilePathBuf, tokio::sync::oneshot::Receiver<filesystem::Result<()>>)> {
        // FIXME: A NamedTempFile would be enough here. No need to create temporary filesystem
        let tmp_filesystem = FilesystemLocalTemp::default();

        // Execute all the processing
        // FIXME: If it is a GIF or some other extension that will loose something (animation,
        // FIXME: video, ...) when converting to PNG we should raise here. Maybe don't
        // FIXME: convert/optimize and just upload
        let photo = match crate::image::prepare_image_file(original.as_ref(), &tmp_filesystem) {
            Ok(photo) => photo,
            Err(e) => {
                bail!("Error processing file {}: {e}", original.as_ref());
            }
        };

        // Calculate the filepath based on photo properties
        // TODO: Instead of using a sha256-based storage path, it would be great to use one based
        // TODO: on datetime when the photo was taken: `<year>/<month>/<day>/...` so it can be
        // TODO: browsed in pCloud as well.
        let filepath = {
            // Compute remote path by chunking sha256 string
            let photo_fullpath = tmp_filesystem.resolve_filepath(&photo);
            let sha256 = sha256_string_from_file(photo_fullpath)?;
            debug!(" - sha256 '{}'", sha256);
            let (c1, rest) = sha256.split_at(4);
            let (c2, rest) = rest.split_at(4);

            let path = Utf8Path::new(SHA256_BASE_PATH).join(c1).join(c2);
            let path = DirectoryPathBuf::from_str(path.as_str())?;
            path.join_filename(FilenameBuf::from_str(&format!("{}.png", rest)).unwrap())
        };
        debug!("Upload to '{}'", filepath);

        // Do the upload
        self.storage.create_dir_all(filepath.directory()).await?;
        let rx = filesystem::actions::copy_file(&tmp_filesystem, &mut self.storage, &photo, &filepath, false).await?;

        Ok((filepath, rx.unwrap()))
    }

    pub async fn add(&mut self, photo_filepath: impl AsRef<Utf8Path>) -> Result<()> {
        debug!("Add photo from path '{}'", photo_filepath.as_ref());

        let (remote_filepath, rx) = self.process_and_upload(&photo_filepath).await?;

        debug!("Now create the PhotoFile entry for {}", remote_filepath);
        let mut conn = self.db.get_connection()?;

        use crate::database::schema::formats::dsl::*;
        let format_: models::Format = formats.filter(format.eq("png")).get_result(&mut conn)?;
        // FIXME: Hash/size might not be available right away...
        rx.await??; // Wait for the upload and DB entry creation

        // FIXME: Here we need an absolute path to satisfy pcloud's RemotePath... we need to
        // FIXME: consolidate filesystem with pcloud_sdk (maybe the other way around) so they
        // FIXME: both uses a relative path (whatever is root will always be prepended)
        let remote_abs_filepath = Utf8Path::new("/").join(&remote_filepath);
        debug!("Get fileid for {}", remote_abs_filepath);
        let fileid_: FileID = self
            .pcloud
            .get_fileid(&RemotePath::try_from(remote_abs_filepath)?)
            .await?;

        debug!("Get the models::File row for {}", remote_filepath);
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
        let (tx, rx) = flume::bounded::<Box<dyn FileMetadata>>(32);

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
