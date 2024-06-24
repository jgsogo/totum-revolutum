use anyhow::Result;
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{BoolExpressionMethods, ExpressionMethods};
use diesel::{Connection, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel_utils::managers::AllManager;
use filesystem::impls::composites::{FilesystemIndexedDatabase, FilesystemIndexedDbDirectory};
use filesystem::{DirectoryPath, Error, Filename};
use tokio::sync::oneshot::Receiver;
use tracing::debug;

use crate::database::models;
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

    fn create_directory(
        &self,
        dirname: &DirectoryPath,
        parent_dir: Option<&models::Directory>,
    ) -> filesystem::Result<models::Directory> {
        let parent_dirname = dirname.parent().unwrap_or_else(|| DirectoryPath::root());

        let parent_dir_id = match parent_dir {
            None => {
                let parent_dir = self.get_directory(parent_dirname)?;
                parent_dir.id
            }
            Some(p) => {
                if parent_dirname != p.full_path() {
                    return Err(Error::Other(
                        "Given parent_dir is not a parent of the directory we are creating".to_string(),
                    ));
                }
                p.id
            }
        };

        use super::schema::directories::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let new_directory = models::NewDirectory {
            parent_id: Some(parent_dir_id),
            full_path: dirname.as_str(),
        };
        let dir = diesel::insert_into(directories)
            .values(&new_directory)
            .returning(models::Directory::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(dir)
    }
}

#[async_trait]
impl<PCloud: PCloudClient + Send> Database for PCloudDatabase<PCloud> {
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool.get().map_err(|e| anyhow!(e.to_string()))
    }
}

#[async_trait]
impl<PCloud: PCloudClient + Send> FilesystemIndexedDatabase for PCloudDatabase<PCloud> {
    type File = models::File;
    type Directory = models::Directory;

    fn all_files(&self) -> filesystem::Result<impl Iterator<Item = Self::File>> {
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let all_files =
            models::File::all(models::File::as_select(), &mut conn).map_err(|e| Error::Other(e.to_string()))?;
        let all_files = all_files.collect::<Vec<_>>();
        Ok(all_files.into_iter())
    }

    fn all_directories(&self) -> filesystem::Result<impl Iterator<Item = Self::Directory>> {
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let all_directories = models::Directory::all(models::Directory::as_select(), &mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        let all_directories = all_directories.collect::<Vec<_>>();
        Ok(all_directories.into_iter())
    }

    fn get_files_in_directory(&self, dir: &Self::Directory) -> filesystem::Result<impl Iterator<Item = Self::File>> {
        use super::schema::files::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let files_in_dir: Vec<models::File> = files
            .filter(directory_id.eq(dir.id))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(files_in_dir.into_iter())
    }

    fn get_directory(&self, path: &DirectoryPath) -> filesystem::Result<Self::Directory> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        match directories
            .filter(full_path.eq(path.to_string()))
            .select(models::Directory::as_select())
            .first(&mut conn)
        {
            Ok(record) => Ok(record),
            Err(e) => match e {
                diesel::result::Error::NotFound => Err(Error::PathDoesNotExist),
                _ => Err(Error::Other(e.to_string())),
            },
        }
    }

    fn get_file(&self, dir: &Self::Directory, filename: &Filename) -> filesystem::Result<Self::File> {
        use super::schema::files::dsl::*;

        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let files_found = files
            .filter(name.eq(filename.as_str()).and(directory_id.eq(dir.id)))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        if files_found.is_empty() {
            Err(Error::PathDoesNotExist)
        } else {
            Ok(files_found.into_iter().nth(0).unwrap())
        }
    }

    fn get_or_create_directory(
        &self,
        path: &DirectoryPath,
        parent_dir: Option<&Self::Directory>,
    ) -> filesystem::Result<(Self::Directory, bool)> {
        // Try to get, if found return (any error other than not-found is returned too)
        match self.get_directory(path) {
            Ok(dir) => {
                return Ok((dir, false));
            }
            Err(e) => match e {
                Error::PathDoesNotExist => {}
                _ => {
                    return Err(Error::Other(e.to_string()));
                }
            },
        }

        // Not found, we need to create it
        let r = self.create_directory(path, parent_dir)?;
        Ok((r, true))
    }

    fn delete_file(&self, file: Self::File) -> filesystem::Result<()> {
        use super::schema::files::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        diesel::delete(files.filter(id.eq(file.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn get_directories_in_directory(
        &self,
        dir: &Self::Directory,
    ) -> filesystem::Result<impl Iterator<Item = Self::Directory>> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        let directories_in_dir: Vec<models::Directory> = directories
            .filter(parent_id.eq(dir.id))
            .select(models::Directory::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(directories_in_dir.into_iter())
    }

    fn delete_directory(&self, dir: Self::Directory) -> filesystem::Result<()> {
        use super::schema::directories::dsl::*;
        let mut conn = self.get_connection().map_err(|e| Error::Other(e.to_string()))?;
        diesel::delete(directories.filter(id.eq(dir.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn delete_directory_on_cascade(&self, dir: Self::Directory) -> filesystem::Result<()> {
        // TODO: Detect if DELETE ON CASCADE if activated for this DB
        self.delete_directory(dir)
    }

    fn upsert_file(
        &self,
        _dir: &Self::Directory,
        _filename: &Filename,
        _new_size: Option<u64>,
        _new_hash: Option<&str>,
    ) -> filesystem::Result<()> {
        todo!()
    }

    fn update_file(
        &self,
        _file: Self::File,
        _new_directory: Option<&Self::Directory>,
        _new_filename: Option<&Filename>,
        _new_size: Option<u64>,
        _new_hash: Option<&str>,
    ) -> filesystem::Result<Self::File> {
        todo!()
    }

    fn create_file(
        &self,
        _dir: &Self::Directory,
        _filename: &Filename,
        _hash_: &str,
        _size_: i32,
    ) -> filesystem::Result<Self::File> {
        todo!()
    }
}
