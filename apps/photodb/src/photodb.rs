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
    /// the [`models::File`] that are not a [`models::PhotoFile`] or VideoFile and process the
    /// files to create the corresponding entries.
    ///
    /// Note that, for the files that have been removed from the storage, the `mirror` operation
    /// will remove the [`models::File`] and ON CASCADE the corresponding [`models::PhotoFile`] or
    /// VideoFile will be removed.
    pub async fn sync(&self) -> Result<()> {
        self.storage.initial_sync().await.map_err(|e| anyhow!(e))

        // TODO: Iterate all the [`models::File`] that are not a [`models::PhotoFile`] or VideoFile
        // TODO: and process the files to create the corresponding entries
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

        if let Err(e) = self.storage.walk_directory(tx, 10, Utf8Path::new("<not used>")).await {
            error!("Error iterating storage files: {e}");
        }

        Ok(())
    }

    // pub async fn clean_fileids(&self) -> Result<()> {
    //     // Iterate all the entires in the photos table, check if the corresponding file_id exists,
    //     // remove if it doesn't
    //
    //     let (db_remove_tx, mut db_remove_rx) = tokio::sync::mpsc::channel(100);
    //
    //     let mut conn = self.db.get_connection()?;
    //     tokio::spawn(async move {
    //         loop {
    //             // TODO: Hide receiver behind an iterator, take a batch
    //             match db_remove_rx.recv().await {
    //                 None => break,
    //                 Some(row_id) => {
    //                     if let Err(e) = diesel::delete(photos.filter(id.eq(row_id))).execute(&mut conn) {
    //                         error!("Error removing row {row_id}: {e}");
    //                     }
    //                 }
    //             }
    //         }
    //     });
    //
    //     // TODO: Write some abstraction to use an iterator (pagination hidden) to iterate over the full table. See https://github.com/diesel-rs/diesel/issues/1087
    //     use crate::schema::photos::dsl::*;
    //     let results = photos.select(Photo::as_select()).load(&mut self.db.get_connection()?)?;
    //
    //     // Create tokio tasks, so they can run in parallel
    //     let mut set = tokio::task::JoinSet::new();
    //
    //     for r in results {
    //         let tx = db_remove_tx.clone();
    //         let pcloud = self.pcloud.clone();
    //         set.spawn(async move {
    //             let fileid_ = FileID::new(r.fileid as u64);
    //             debug!("Check if '{fileid_}' exists");
    //             match pcloud.stat(fileid_.clone().into()).await {
    //                 Ok(_) => None,
    //                 Err(e) => {
    //                     debug!("Error for {fileid_}: {e}");
    //                     if let Error::PCloudError { code, .. } = e {
    //                         if code == 2009 {
    //                             tx.send(r.id).await.ok();
    //                             Some(r.id)
    //                         } else {
    //                             None
    //                         }
    //                     } else {
    //                         None
    //                     }
    //                 }
    //             }
    //         });
    //     }
    //
    //     while let Some(res) = set.join_next().await {
    //         match res {
    //             Ok(Some(id_)) => {
    //                 debug! {"Requested removal of photo.id {id_}"}
    //             }
    //             Ok(None) => {}
    //             Err(e) => error!("Error running task: {e}"),
    //         }
    //     }
    //     Ok(())
    // }
}
