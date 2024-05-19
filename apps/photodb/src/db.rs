use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use camino::Utf8PathBuf;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use dotenvy::dotenv;
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::{GetCreateFolderIfNotExistsAll, GetFileLinkAndDownload, GetFolderID};
use pcloud_sdk::methods::file::stat::GetStat;
use pcloud_sdk::methods::fileops::file_close::GetFileClose;
use pcloud_sdk::methods::fileops::file_open::{FileOpenPath, Flags, GetFileOpen};
use pcloud_sdk::methods::fileops::file_write::PostFileWrite;
use pcloud_sdk::methods::streaming::getfilelink::GetFileLinkInput;
use pcloud_sdk::progress_bar::ProgressBarBuilder;
use pcloud_sdk::types::{FileID, FolderID, RemotePath};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::str::FromStr;
use tokio::sync::oneshot::Sender;
use tracing::{debug, error};

// TODO: We need a proper Output for this PhotoDB application
#[derive(Default)]
struct Output;
impl ProgressBarBuilder for Output {}

const DB_FILENAME: &str = "photodb.sqlite3";

async fn download<PCloud: GetFileLinkAndDownload>(
    pcloud: &PCloud,
    local_filename: &Utf8PathBuf,
    remote_file: FileID,
) -> Result<()> {
    debug!("Download remote DB file '{remote_file}' to '{local_filename}'");
    // TODO: Implement and use pcloud.downloadfile

    let file_link = GetFileLinkInput::new(pcloud_sdk::types::File::from(remote_file));
    pcloud
        .getfilelink_and_download(file_link, local_filename, &Output)
        .await
}

async fn upload<PCloud: GetFileClose + PostFileWrite + GetFileOpen>(
    pcloud: &PCloud,
    mut file: File,
    folder: FolderID,
) -> Result<()> {
    debug!("Upload DB from local file to folder '{folder}'");

    // Open the remote file
    let fd = pcloud
        .file_open(
            Flags::O_CREAT | Flags::O_WRITE | Flags::O_TRUNC | Flags::O_APPEND,
            FileOpenPath::FolderAndName(folder.clone(), DB_FILENAME.to_string()),
        )
        .await?;

    // Read content from the local file
    let mut buffer = Vec::new();
    let bytes_read = file.read_to_end(&mut buffer)?;

    // Write the buffer into the remote file
    let bytes_count = pcloud.file_write(fd.fd, &buffer).await?;
    assert_eq!(bytes_count.bytes, bytes_read as u64);

    // Close and done
    pcloud.file_close(fd.fd).await
}

fn temp_db_filename(app_dir: &Utf8PathBuf) -> Utf8PathBuf {
    let datetime = chrono::offset::Local::now().format("%Y%m%y_%H%M%S");
    app_dir.join(format!("{datetime}.sqlite3"))
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
    tmpfile: Utf8PathBuf,
    pool: Pool<ConnectionManager<SqliteConnection>>,
    drop_tx: Option<Sender<(PCloud, Utf8PathBuf, FolderID)>>,
}

impl<PCloud: PCloudClient + Send + 'static> PCloudDatabase<PCloud> {
    /// Initializes the database and pushes it to the remote pCloud storage. It will fail if the
    /// remote file already exists
    pub async fn initialize(pcloud: PCloud, app_dir: &Utf8PathBuf, path: RemotePath) -> Result<()> {
        // Create the remote folder and check if file exists
        let folderid = pcloud.createfolderifnotexists_all(None, &path).await?;
        let db_remote_file = RemotePath::from_str(&format!("{}/{}", path, DB_FILENAME))?;
        if pcloud
            .stat(pcloud_sdk::types::File::try_from(db_remote_file)?)
            .await
            .is_ok()
        {
            bail!("Remote DB already exists");
        }

        // Create the database in a local file
        let tmpfilename = temp_db_filename(app_dir);

        // TODO: Create the SQLite3 database
        {
            let mut file = File::create(&tmpfilename)?;
            file.write_all(b"Hello, world!")?;
        }

        // Upload local file to remote
        let file = File::open(&tmpfilename)?;
        upload(&pcloud, file, folderid.clone()).await?;

        // Remove local file once we are done
        std::fs::remove_file(&tmpfilename)?;
        Ok(())
    }

    /// Creates a new [`PCloudDatabase`] instance. Requires a pCloud client and the folder
    /// path where the database (and files) are located
    pub async fn new(pcloud: PCloud, app_dir: &Utf8PathBuf, path: RemotePath) -> Result<Self> {
        // Get the relevant IDs from pCloud
        debug!("Use remote (pcloud) filesystem at '{path}'");
        let folderid = pcloud.get_folderid(&path).await?;
        let db_remote_file = RemotePath::from_str(&format!("{}/{}", path, DB_FILENAME))?;
        let fileid = pcloud
            .stat(pcloud_sdk::types::File::try_from(db_remote_file)?)
            .await?
            .metadata
            .fileid;

        // Download the database to a temporary file
        debug!("Download the DB file");
        let tmpfilename = temp_db_filename(app_dir);
        download(&pcloud, &tmpfilename, fileid.clone()).await?;

        // Spawn a task so we can execute async drop: ensure DB is finally uploaded
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::task::spawn(async move {
            match rx.await {
                Ok((pcloud_, filepath_, folderid_)) => {
                    let f = File::open(&filepath_).unwrap_or_else(|_| panic!("Failed to open filepath '{filepath_}'"));
                    upload(&pcloud_, f, folderid_).await.expect("Failed to upload");
                    std::fs::remove_file(&filepath_).expect("Upload succeeded, but failed to remove file");
                }
                Err(e) => error!("Error receiving final upload: {e}"),
            }
        });

        // Create a connection pool using the local temp file
        let pool = {
            let manager = ConnectionManager::<SqliteConnection>::new(tmpfilename.clone());
            Pool::builder().test_on_check_out(true).build(manager)?
        };
        // Return the instance
        Ok(Self {
            pcloud: Some(pcloud),
            pcloud_folderid: folderid,
            tmpfile: tmpfilename,
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
        let f = File::open(&self.tmpfile)?;
        let _ = upload(self.pcloud.as_ref().unwrap(), f, self.pcloud_folderid.clone()).await?;
        Ok(())
    }
}

impl<PCloud: PCloudClient + Send> Drop for PCloudDatabase<PCloud> {
    fn drop(&mut self) {
        let data = (
            self.pcloud.take().unwrap(),
            self.tmpfile.clone(),
            self.pcloud_folderid.clone(),
        );
        if self.drop_tx.take().unwrap().send(data).is_err() {
            error!("Error sending drop signal");
        }
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
