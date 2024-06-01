use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use pcloud_sdk::client::PCloudClient;
use pcloud_sdk::handy::GetFolderID;
use pcloud_sdk::progress_bar::ProgressBarBuilder;
use pcloud_sdk::types::RemotePath;
use pcloud_sdk::ProxiedFile;
use std::path::PathBuf;
use tempfile::TempDir;
use tokio::sync::oneshot::Receiver;
use tracing::debug;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

// TODO: We need a proper Output for this PhotoDB application
#[derive(Default)]
struct Output;
impl ProgressBarBuilder for Output {}

const DB_FILENAME: &str = "photodb.sqlite3";

/// Manages the connection to the database (Sqlite3)
#[async_trait]
pub trait Database {
    /// Returns a connection to the database
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>>;

    // ///Executes any pending operation
    // async fn flush(&self) -> Result<()>;
}

pub struct PCloudDatabase<PCloud: PCloudClient + Send + 'static> {
    pool: Pool<ConnectionManager<SqliteConnection>>,
    _proxied_file: ProxiedFile<PCloud>,
}

impl<PCloud: PCloudClient + Send + Clone + 'static> PCloudDatabase<PCloud>
where
    PCloud: http_utils::HttpClient<Error = pcloud_sdk::Error>,
{
    /// Initializes the database and pushes it to the remote pCloud storage. It will fail if the
    /// remote file already exists
    pub async fn initialize(pcloud: PCloud, path: RemotePath) -> Result<Receiver<Result<(), (TempDir, PathBuf)>>> {
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
    pub async fn new(pcloud: PCloud, path: RemotePath) -> Result<(Self, Receiver<Result<(), (TempDir, PathBuf)>>)> {
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
