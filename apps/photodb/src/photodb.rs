use super::db::Database;
use super::models;
use super::utils::sha256_string_from_file;
use super::AppDirs;
use anyhow::{anyhow, bail, Result};
use camino::Utf8PathBuf;
use diesel::{RunQueryDsl, SelectableHelper};
use oxipng::{optimize, Options};
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::Exists;
use pcloud_sdk::handy::GetCreateFolderIfNotExistsAll;
use pcloud_sdk::methods::file::uploadfile::{PostUploadFile, UploadFileParams};
use pcloud_sdk::types::{FileID, Folder, RemotePath};
use std::str::FromStr;
use tracing::{debug, info};

// Path inside the remote folder to locate files using sha256 filename
const SHA256_BASE_PATH: &str = "_sha256";

#[allow(dead_code)]
pub struct PhotoDB<'a, T: Database, PCloud: PCloudClient> {
    pcloud: PCloud,
    db: T,
    app_dir: &'a AppDirs,
    remote_dir: RemotePath,
}

impl<'a, T: Database, PCloud: PCloudClient> PhotoDB<'a, T, PCloud> {
    pub fn new(db: T, pcloud: PCloud, app_dir: &'a AppDirs, remote_dir: RemotePath) -> Self {
        info!(
            "New photodb application using local directory '{}' and remote directory '{}'",
            app_dir, remote_dir
        );
        Self {
            pcloud,
            db,
            app_dir,
            remote_dir,
        }
    }

    fn to_tmp_storage(&self, input: Utf8PathBuf) -> Result<Utf8PathBuf> {
        debug!("Convert to PNG format");
        let input = {
            let image = image::io::Reader::open(input)?.decode()?;
            let input_filename = self.app_dir.temp_filename(None, None);
            image.save_with_format(&input_filename, image::ImageFormat::Png)?;
            input_filename
        };

        debug!("Apply oxipng optimizer");
        let output_filename = {
            let input_file = oxipng::InFile::Path(input.into_std_path_buf());

            let tmp_filename = self.app_dir.temp_filename(None, None);
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
        // FIXME: If it is a GIF or some other extension that will loose something (animation,...)
        // FIXME: when converter to PNG we should raise here. Maybe don't convert/optimize and just
        // FIXME: upload

        let photo = self.to_tmp_storage(photo_filepath)?;

        // Upload to remote
        // TODO: Instead of using a sha256-based storage path, it would be great to use one based
        // TODO: on datetime when the photo was taken: `<year>/<month>/<day>/...` so it can be
        // TODO: browsed in pCloud as well.
        let (folder, filename) = {
            // Compute remote path by chunking sha256 string
            let sha256 = sha256_string_from_file(&photo)?;
            debug!(" - sha256 '{}'", sha256);
            let (c1, rest) = sha256.split_at(4);
            let (c2, rest) = rest.split_at(4);
            let folder_path = self.remote_dir.path().join(SHA256_BASE_PATH).join(c1).join(c2);
            (
                RemotePath::from_str(&format!("path:/{}", folder_path))?,
                format!("{}.png", rest),
            )
        };
        debug!("Upload to '{}/{}'", folder, filename);
        let folderid = self.pcloud.createfolderifnotexists_all(None, &folder).await?;

        // FIXME: Handle file sha256 collision
        let exists = self.pcloud.exists(folderid, &filename).await?;
        if exists.is_some() {
            bail!("A file with the same sha256 already exists!");
        }

        let r = self
            .pcloud
            .uploadfile(&photo, UploadFileParams::new(Folder::RemotePath(folder), filename))
            .await?;
        let file_id = FileID(*r.fileids.first().unwrap());

        // Store the data in the database
        use crate::schema::photos;
        let new_photo = models::NewPhoto {
            fileid: &(file_id.0 as i64),
        };
        let photo = diesel::insert_into(photos::table)
            .values(&new_photo)
            .returning(models::Photo::as_returning())
            .get_result(&mut self.db.get_connection()?)?;

        debug!("Photo inserted into database: {}", photo.id);

        Ok(())
    }
}
