use std::str::FromStr;

use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use log::error;
use oxipng::{optimize, Options};
use tracing::{debug, info};

use filesystem::impls::composites::indexed::diesel_indexed::DatabaseImpl;
use filesystem::impls::composites::FilesystemIndexed;
use filesystem::impls::{FilesystemLocalTemp, FilesystemPCloud};
use filesystem::{DirectoryPathBuf, FileMetadata, FilePathBuf, FilenameBuf, Filesystem};
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

    fn prepare_image_file(input: Utf8PathBuf, filesystem_local: &FilesystemLocalTemp) -> Result<FilePathBuf> {
        debug!("Convert to PNG format");
        let input = {
            let image = image::io::Reader::open(input)?.decode()?;
            let input_filename = tempfile::NamedTempFile::new()?.into_temp_path();
            image.save_with_format(&input_filename, image::ImageFormat::Png)?;
            input_filename
        };

        debug!("Apply oxipng optimizer");
        let output_filename = {
            let input_file = oxipng::InFile::Path(input.to_path_buf());

            let tmp_filename = filesystem_local.temp_filename(None, None);
            let output_file = oxipng::OutFile::from_path(tmp_filename.as_std_path().to_path_buf());

            optimize(&input_file, &output_file, &Options::from_preset(2))
                .map_err(|e| anyhow!("Error converting image: {e}"))?;
            debug!("Input photo optimized and saved to tmp file '{}'", tmp_filename);
            tmp_filename
        };

        Ok(output_filename)
    }

    /// Takes a [`models::File`] and creates (or updates) the corresponding [`models::PhotoFile`].
    ///
    /// The file will unconditionally go through all the processes performed by PhotoDB application
    /// and it will potentially be moved to a different location
    fn process_stored_file(&self, _file: &models::File) -> Result<()> {
        // Copy from storage to local folder
        // Execute all the processing
        // Upload to remote (compute new paths)
        // TODO: Not implemented
        Ok(())
    }

    pub async fn add(&mut self, photo_filepath: Utf8PathBuf) -> Result<()> {
        debug!("Add photo at '{}'", photo_filepath);
        // FIXME: If it is a GIF or some other extension that will loose something (animation,
        // FIXME: video, ...) when converting to PNG we should raise here. Maybe don't
        // FIXME: convert/optimize and just upload

        let tmp_filesystem = FilesystemLocalTemp::default();
        let photo = Self::prepare_image_file(photo_filepath, &tmp_filesystem)?;

        // Upload to remote
        // TODO: Instead of using a sha256-based storage path, it would be great to use one based
        // TODO: on datetime when the photo was taken: `<year>/<month>/<day>/...` so it can be
        // TODO: browsed in pCloud as well.
        let filepath = {
            // Compute remote path by chunking sha256 string
            let sha256 = sha256_string_from_file(&photo)?;
            debug!(" - sha256 '{}'", sha256);
            let (c1, rest) = sha256.split_at(4);
            let (c2, rest) = rest.split_at(4);

            let path = Utf8Path::new(SHA256_BASE_PATH).join(c1).join(c2);
            let path = DirectoryPathBuf::from_str(path.as_str())?;
            path.join_filename(FilenameBuf::from_str(&format!("{}.png", rest)).unwrap())
        };
        debug!("Upload to '{}'", filepath);
        debug!("Create intermediate directories '{}'", filepath.directory());
        self.storage.create_dir_all(filepath.directory()).await?;
        filesystem::actions::copy_file(&tmp_filesystem, &mut self.storage, &photo, &filepath, false).await?;

        let mut conn = self.db.get_connection()?;

        use crate::database::schema::formats::dsl::*;
        use crate::database::schema::photo_files::dsl::*;

        let file_ = self.db.get_file(&filepath)?;
        let fileid_: FileID = self
            .pcloud
            .get_fileid(&RemotePath::try_from(filepath.as_utf8_path())?)
            .await?;
        let format_: models::Format = formats.filter(format.eq("png")).get_result(&mut conn)?;
        let new_photo_file = models::PhotoFile::new_from(&file_, &fileid_, &format_, true);
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
    pub async fn sync(&self) -> Result<()> {
        let ignore_filter = self.app_dir.ignore_filters().await?;
        self.storage.initial_sync(ignore_filter).await.map_err(|e| anyhow!(e))?;

        let non_identified = self.db.get_orphan_files()?;
        debug!("Found {} files not processed", non_identified.len());
        for it in non_identified {
            self.process_stored_file(&it)?;
        }
        Ok(())
    }

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
