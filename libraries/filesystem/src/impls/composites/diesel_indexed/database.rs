use crate::{File, FileMetadata, Filesystem, FilesystemRead, FilesystemRemove, FilesystemWrite, Result};
use async_trait::async_trait;
use camino::Utf8Path;
use flume::Sender;
use tokio::sync::oneshot::Receiver;

// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
// const MIGRATIONS: EmbeddedMigrations = embed_migrations!("impls/composites/diesel_indexed/migrations");

pub struct Database;

impl Database {
    pub fn new(_database_url: &str) -> Result<Self> {
        // // Create a connection pool using the local temp file
        // let pool = {
        //     let manager = ConnectionManager::<SqliteConnection>::new(proxied_file.local_filepath().to_str().unwrap());
        //     Pool::builder().test_on_check_out(true).build(manager)?
        // };
        //
        // let mut conn = pool.get()?;
        // conn.run_pending_migrations(MIGRATIONS)
        //     .map_err(|e| anyhow!("Error {}", e))?;
        //
        Ok(Self)
    }
}

#[async_trait]
impl Filesystem for Database {
    async fn sync_all(self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl FilesystemRead for Database {
    async fn walk_directory(
        &self,
        _tx: Sender<Box<dyn FileMetadata>>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        todo!()
    }

    async fn get_metadata(&self, _path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        todo!()
    }

    async fn exists(&self, _path: &Utf8Path) -> Result<bool> {
        todo!()
    }

    async fn open(&self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        todo!()
    }
}

#[async_trait]
impl FilesystemWrite for Database {
    async fn create(&self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        todo!()
    }

    async fn create_dir_all(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }
}

#[async_trait]
impl FilesystemRemove for Database {
    async fn remove_file(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }

    async fn remove_dir(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }

    async fn remove_dir_all(&self, _path: &Utf8Path) -> Result<()> {
        todo!()
    }
}
