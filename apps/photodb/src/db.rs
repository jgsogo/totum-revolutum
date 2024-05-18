use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenvy::dotenv;
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::{GetCreateFolderIfNotExistsAll, GetFileLinkAndDownload, GetFolderID};
use pcloud_sdk::methods::file::stat::GetStat;
use pcloud_sdk::methods::file::uploadfile::{PostUploadFile, UploadFile, UploadFileParams};

use pcloud_sdk::methods::streaming::getfilelink::GetFileLinkInput;
use pcloud_sdk::progress_bar::ProgressBarBuilder;
use pcloud_sdk::types::{File, FileID, Folder, FolderID, RemotePath};
use std::env;
use std::str::FromStr;
use tempfile::NamedTempFile;
use tokio::sync::oneshot::Sender;
use tracing::{debug, error};

// TODO: We need a proper Output for this PhotoDB application
#[derive(Default)]
struct Output;
impl ProgressBarBuilder for Output {}

const DB_FILENAME: &str = "photodb.sqlite3";

async fn download<PCloud: GetFileLinkAndDownload>(
    pcloud: &PCloud,
    tmpfile: &NamedTempFile,
    remote_file: FileID,
) -> Result<()> {
    let local_path = Utf8Path::from_path(tmpfile.path()).ok_or(anyhow!("Cannot get "))?;
    debug!("Download remote DB file '{remote_file}' to '{local_path}'");
    // TODO: Implement and use pcloud.downloadfile

    let file_link = GetFileLinkInput::new(File::from(remote_file));
    pcloud.getfilelink_and_download(file_link, local_path, &Output).await
}

async fn upload<PCloud: PostUploadFile>(
    pcloud: &PCloud,
    tmpfile: &NamedTempFile,
    folder: FolderID,
) -> Result<UploadFile> {
    let local_path = Utf8Path::from_path(tmpfile.path()).ok_or(anyhow!("Cannot get "))?;
    debug!("Upload DB from '{local_path}' to folder '{folder}'");
    let params = UploadFileParams::new(Folder::from(folder), DB_FILENAME.to_string());
    pcloud.uploadfile(local_path, params).await
}

/// Manages the connection to the database (Sqlite3)
#[async_trait]
pub trait Database {
    /// Returns a connection to the database
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>>;

    /// Executes any pending operation
    async fn flush(&self) -> Result<()>;
}

pub struct PCloudDatabase<PCloud: PCloudClient + Send> {
    pcloud: Option<PCloud>,
    pcloud_folderid: FolderID,
    _pcloud_fileid: FileID,
    tmpfile: Option<NamedTempFile>,
    pool: Pool<ConnectionManager<SqliteConnection>>,
    drop_tx: Option<Sender<(PCloud, NamedTempFile)>>,
}

impl<PCloud: PCloudClient + Send + 'static> PCloudDatabase<PCloud> {
    /// Initializes the database and pushes it to the remote pCloud storage. It will fail if the
    /// remote file already exists
    pub async fn initialize(pcloud: PCloud, path: RemotePath) -> Result<()> {
        // Create the database in a local file
        let tmpfile = NamedTempFile::new()?;

        // TODO: Create the SQLite3 database

        // Create the folder and upload the file
        let folderid = pcloud.createfolderifnotexists_all(None, &path).await?;
        let db_remote_file = RemotePath::from_str(&format!("{}/{}", path, DB_FILENAME))?;
        if pcloud.stat(File::try_from(db_remote_file)?).await.is_ok() {
            bail!("Remote DB already exists");
        }
        let _r = upload(&pcloud, &tmpfile, folderid.clone()).await?;
        Ok(())
    }

    /// Creates a new [`PCloudDatabase`] instance. Requires a pCloud client and the folder
    /// path where the database (and files) are located
    pub async fn new(pcloud: PCloud, path: RemotePath) -> Result<Self> {
        // We will use a temporary local file while using the DB
        let tmpfile = NamedTempFile::new()?;

        // Get the relevant IDs from pCloud
        debug!("Use remote (pcloud) filesystem at '{path}'. Get folder and file identifiers");
        let folderid = pcloud.get_folderid(&path).await?;
        let db_remote_file = RemotePath::from_str(&format!("{}/{}", path, DB_FILENAME))?;
        let fileid = pcloud.stat(File::try_from(db_remote_file)?).await?.metadata.fileid;

        // Download the database
        debug!("Download the DB file");
        download(&pcloud, &tmpfile, fileid.clone()).await?;

        // Spawn a task so we can execute async drop: ensure DB is finally uploaded
        let (tx, rx) = tokio::sync::oneshot::channel();
        let folderid4drop = folderid.clone();
        let tmpfile_path = Utf8PathBuf::from_path_buf(tmpfile.path().to_path_buf())
            .expect("Error converting temporary file path to Utf8");
        tokio::task::spawn(async move {
            let r = match rx.await {
                Ok((p, f)) => {
                    debug!("Drop signal received for PCloudDatabase instance");
                    let _ = upload(&p, &f, folderid4drop).await;
                    Ok(())
                }
                Err(_e) => Err(anyhow!("Failed receive")),
            };
            if r.is_err() {
                // TODO: Send database to some persistent storage
                error!("Failed to upload and sync DB");
            }
        });

        // Create a connection pool using the local temp file
        let pool = {
            let path_string: String = tmpfile_path.as_str().into();
            let manager = ConnectionManager::<SqliteConnection>::new(path_string);
            Pool::builder().test_on_check_out(true).build(manager)?
        };
        // Return the instance
        Ok(Self {
            pcloud: Some(pcloud),
            pcloud_folderid: folderid,
            _pcloud_fileid: fileid,
            tmpfile: Some(tmpfile),
            pool,
            drop_tx: Some(tx),
        })
    }
}

#[async_trait]
impl<PCloud: PCloudClient + Send> Database for PCloudDatabase<PCloud> {
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool.get().map_err(|e| anyhow!(e.to_string()))
    }

    async fn flush(&self) -> Result<()> {
        debug!("Flush DB remote (pcloud)");
        let _ = upload(
            self.pcloud.as_ref().unwrap(),
            self.tmpfile.as_ref().unwrap(),
            self.pcloud_folderid.clone(),
        )
        .await?;
        Ok(())
    }
}

impl<PCloud: PCloudClient + Send> Drop for PCloudDatabase<PCloud> {
    fn drop(&mut self) {
        // FIXME: Handle error: copy DB to persistent storage and pray
        let _r = self
            .drop_tx
            .take()
            .unwrap()
            .send((self.pcloud.take().unwrap(), self.tmpfile.take().unwrap()));
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
