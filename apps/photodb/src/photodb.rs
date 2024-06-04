use anyhow::{anyhow, Result};
use camino::{Utf8Path, Utf8PathBuf};
use diesel::{RunQueryDsl, SelectableHelper};
// use diesel::prelude::*;
use oxipng::{optimize, Options};
use tracing::{debug, info};

use filesystem::local_temp::FilesystemLocalTemp;
use filesystem::Filesystem;
use filesystem_pcloud::RemoteMetadata;

use super::db::Database;
use super::models;
use super::models::Photo;
use super::utils::sha256_string_from_file;
use super::AppDirs;

// Path inside the remote folder to locate files using sha256 filename
const SHA256_BASE_PATH: &str = "_sha256";

#[allow(dead_code)]
pub struct PhotoDB<'a, T: Database, RemoteStorage: Filesystem<Metadata = RemoteMetadata>> {
    db: T,
    app_dir: &'a AppDirs,
    storage: RemoteStorage,
}

impl<'a, T: Database, RemoteStorage: Filesystem<Metadata = RemoteMetadata>> PhotoDB<'a, T, RemoteStorage> {
    pub async fn new(db: T, storage: RemoteStorage, app_dir: &'a AppDirs) -> Result<Self> {
        info!(
            "New photodb application using local directory '{}' and remote storage",
            app_dir
        );
        Ok(Self { db, app_dir, storage })
    }

    fn prepare_image_file(input: Utf8PathBuf, filesystem_local: &FilesystemLocalTemp) -> Result<Utf8PathBuf> {
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
            let output_file = oxipng::OutFile::from_path(tmp_filename.clone().into_std_path_buf());

            optimize(&input_file, &output_file, &Options::from_preset(2))
                .map_err(|e| anyhow!("Error converting image: {e}"))?;
            debug!("Input photo optimized and saved to tmp file '{}'", tmp_filename);
            tmp_filename
        };

        Ok(output_filename)
    }

    pub async fn add(&self, photo_filepath: Utf8PathBuf) -> Result<()> {
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
            Utf8Path::new(SHA256_BASE_PATH)
                .join(c1)
                .join(c2)
                .join(format!("{}.png", rest))
        };
        debug!("Upload to '{}'", filepath);
        filesystem::actions::copy_file(&tmp_filesystem, &self.storage, &photo, &filepath, false).await?;
        let metadata = self.storage.get_metadata(&filepath).await?;

        // Store the data in the database
        use crate::schema::photos;
        let new_photo = models::NewPhoto {
            fileid: &(metadata.fileid().inner() as i64),
            path: filepath.as_str(),
        };
        let photo = diesel::insert_into(photos::table)
            .values(&new_photo)
            .returning(Photo::as_returning())
            .get_result(&mut self.db.get_connection()?)?;
        // TODO: Handle scenario if the insert fails: duplicate fileid

        debug!("Photo inserted into database: {}", photo.id);

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
