use anyhow::Result;
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::sync::oneshot::Receiver;
use tracing::debug;

use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::GetFolderID;
use pcloud_sdk::types::RemotePath;
use pcloud_sdk::{ProxiedFile, UploadReturnType};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
const DB_FILENAME: &str = "photodb.sqlite3";

/// Manages the connection to the database (Sqlite3)
#[async_trait]
pub trait Database {
    /// Returns a connection to the database
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>>;

    // ///Executes any pending operation
    // async fn flush(&self) -> Result<()>;
}

/// The SQLite3 database used by PhotoDB application. The database file is stored in PCloud and it
/// is retrieved when this object is instantiated. On drop the database file is backed-up to the
/// PCLoud file (see [`ProxiedFile`]).
pub struct PCloudDatabase<PCloud: PCloudClient + Send + 'static> {
    pool: Pool<ConnectionManager<SqliteConnection>>,
    _proxied_file: ProxiedFile<PCloud>,
}

impl<PCloud: PCloudClient + Send + 'static> PCloudDatabase<PCloud> {
    /// Initializes the database and pushes it to the remote pCloud storage at the given `path`
    /// location (it creates a file called `photodb.sqlite3` inside the folder). It will fail if
    /// the remote file already exists
    pub async fn initialize(pcloud: PCloud, path: RemotePath) -> Result<Receiver<UploadReturnType>> {
        let folderid = pcloud.get_folderid(&path).await?; // TODO: Create if not exists?
        let (proxied_file, created, upload_done) = ProxiedFile::new(pcloud, folderid, DB_FILENAME).await?;
        if !created {
            bail!("Remote file already exists!");
        }

        // Create the SQLite3 database and run migrations
        debug!("Create the database and/or run pending migrations");
        let mut conn = SqliteConnection::establish(proxied_file.local_filepath().to_str().unwrap())?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("Error {}", e))?;

        Ok(upload_done)
    }

    /// Creates a new [`PCloudDatabase`] instance. Requires a pCloud client and the folder
    /// path where the database (and files) are located
    pub async fn new(pcloud: PCloud, path: RemotePath) -> anyhow::Result<(Self, Receiver<UploadReturnType>)> {
        let folderid = pcloud.get_folderid(&path).await?;
        // TODO: Add a flag to `ProxiedFile` to indicate if it's allowed to create the file or not
        let (proxied_file, _created, upload_done) = ProxiedFile::new(pcloud, folderid, DB_FILENAME).await?;

        // Create a connection pool using the local temp file
        let pool = {
            let manager = ConnectionManager::<SqliteConnection>::new(proxied_file.local_filepath().to_str().unwrap());
            Pool::builder().test_on_check_out(true).build(manager)?
        };

        let mut conn = pool.get()?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow!("Error {}", e))?;

        // Return the instance
        Ok((
            Self {
                pool,
                _proxied_file: proxied_file,
            },
            upload_done,
        ))
    }
}

#[async_trait]
impl<PCloud: PCloudClient + Send> Database for PCloudDatabase<PCloud> {
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool.get().map_err(|e| anyhow!(e.to_string()))
    }
}
