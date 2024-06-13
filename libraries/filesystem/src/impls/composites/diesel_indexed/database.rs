use std::fmt::Debug;

use async_trait::async_trait;
use camino::{Utf8Path, Utf8PathBuf};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use flume::Sender;
use tokio::sync::oneshot::Receiver;

use diesel_utils::managers::AllManager;

use crate::impls::composites::diesel_indexed::models;
use crate::{Error, File, FileMetadata, Filesystem, FilesystemRead, FilesystemRemove, FilesystemWrite, Result};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/impls/composites/diesel_indexed/migrations");

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self> {
        // Create a connection pool using the local temp file
        let pool = {
            let manager = ConnectionManager::<SqliteConnection>::new(database_url);
            Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .map_err(|e| Error::Other(format!("Failed to create the connection pool: {e}")))?
        };

        let mut conn = pool
            .get()
            .map_err(|e| Error::Other(format!("Failed to get one connection from the pool: {e}")))?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| Error::Other(format!("Failed to connect to DB: {e}")))?;

        Ok(Self { pool })
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
        tx: Sender<Box<dyn FileMetadata>>,
        _threads: usize,
        _custom_ignore_filename: &Utf8Path,
    ) -> Result<()> {
        use super::schema::files::dsl::*;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let mut conn2 = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;

        let all_directories = models::Directory::all(models::Directory::as_select(), &mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        for dir in all_directories {
            let dir_path = Utf8PathBuf::from(dir.full_path);

            let all_files = files
                .filter(directory_id.eq(dir.id))
                .select(models::File::as_select())
                .load(&mut conn2)
                .map_err(|e| Error::Other(e.to_string()))?;
            for file in all_files {
                let db_file = DatabaseFile {
                    path: dir_path.join(file.name),
                    size: file.size as u64,
                    hash: file.hash,
                };
                tx.send(Box::new(db_file)).map_err(|e| Error::Other(e.to_string()))?;
            }
        }
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

#[derive(Debug)]
struct DatabaseFile {
    path: Utf8PathBuf,
    size: u64,
    hash: String,
}

impl FileMetadata for DatabaseFile {
    fn path(&self) -> &Utf8Path {
        &self.path
    }

    fn size(&self) -> Result<u64> {
        Ok(self.size)
    }

    fn hash(&self) -> Result<String> {
        Ok(self.hash.clone())
    }
}
