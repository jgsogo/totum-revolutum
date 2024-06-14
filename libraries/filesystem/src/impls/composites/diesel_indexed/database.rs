use std::fmt::Debug;

use async_trait::async_trait;
use camino::{Utf8Component, Utf8Path, Utf8PathBuf};
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
const ROOT_DIRECTORY: &str = "";

pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self> {
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

        diesel::sql_query("PRAGMA foreign_keys = ON") // Enables foreign keys support: https://www.sqlite.org/foreignkeys.html
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(Self { pool })
    }

    fn get_directory(&self, dirname: &Utf8Path) -> Result<Option<models::Directory>> {
        use super::schema::directories::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let directory = directories
            .filter(full_path.eq(dirname.to_string()))
            .select(models::Directory::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(directory)
    }

    fn create_directory(&self, dirname: &Utf8Path) -> Result<models::Directory> {
        use super::schema::directories::dsl::*;

        let parent_: Option<i32> = match dirname.parent() {
            None => None,
            Some(parent_dir) => {
                let parent = self
                    .get_directory(parent_dir)?
                    .ok_or(Error::Other("Failed to get the parent directory".to_string()))?;
                Some(parent.id)
            }
        };

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let new_directory = models::NewDirectory {
            parent_id: parent_,
            full_path: dirname.as_str(),
        };
        let dir = diesel::insert_into(directories)
            .values(&new_directory)
            .returning(models::Directory::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(dir)
    }

    fn get_or_create_directory(&self, dirname: &Utf8Path) -> Result<(models::Directory, bool)> {
        let directory = self.get_directory(dirname);
        match directory {
            Ok(d) => match d {
                None => {
                    let d = self.create_directory(dirname)?;
                    Ok((d, true))
                }
                Some(d) => Ok((d, false)),
            },
            Err(e) => Err(e),
        }
    }

    fn get_files_in_directory(&self, directory: &models::Directory) -> Result<Vec<models::File>> {
        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        files
            .filter(directory_id.eq(directory.id))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))
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
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;

        let all_directories = models::Directory::all(models::Directory::as_select(), &mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        for dir in all_directories {
            let dir_path = Utf8PathBuf::from(&dir.full_path);

            let all_files = self.get_files_in_directory(&dir)?;
            for file in all_files {
                let db_file = DatabaseFile {
                    path: dir_path.join(file.name),
                    size: file.size as u64,
                    hash: file.hash,
                };
                tx.send(Box::new(db_file)).map_err(|e| Error::Other(e.to_string()))?;
            }
        }
        Ok(())
    }

    async fn get_metadata(&self, path: &Utf8Path) -> Result<Box<dyn FileMetadata>> {
        use super::schema::files::dsl::*;

        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let directory = self
            .get_directory(dirname)?
            .ok_or(Error::Other("Not found".to_string()))?;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let file = files
            .filter(name.eq(filename).and(directory_id.eq(directory.id)))
            .select(models::File::as_select())
            .first(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        let db_file = DatabaseFile {
            path,
            size: file.size as u64,
            hash: file.hash,
        };
        Ok(Box::new(db_file))
    }

    async fn exists(&self, path: &Utf8Path) -> Result<bool> {
        use super::schema::files::dsl::*;

        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));

        let directory = self.get_directory(dirname)?;
        if directory.is_none() {
            return Ok(false);
        }
        let directory: i32 = directory.unwrap().id;

        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let file = files
            .filter(name.eq(filename).and(directory_id.eq(directory)))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(!file.is_empty())
    }

    async fn open(&self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        Err(Error::Forbidden)
    }
}

#[async_trait]
impl FilesystemWrite for Database {
    async fn create(&mut self, _path: &Utf8Path) -> Result<(Box<dyn File>, Option<Receiver<Result<()>>>)> {
        // I need all the FileMetadata information from the file
        Err(Error::Forbidden)
    }

    async fn create_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        let path = self.check_path(path)?;
        let mut current_path = Utf8Path::new(ROOT_DIRECTORY).to_path_buf();
        for it in path.components() {
            match it {
                Utf8Component::Normal(c) => {
                    current_path = current_path.join(c);
                    // FIXME: Each call to `get_or_create_directory` is running another call to
                    // FIXME: get the `id` of the parent directory.
                    let _ = self.get_or_create_directory(&current_path)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

#[async_trait]
impl FilesystemRemove for Database {
    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        use super::schema::files::dsl::*;

        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let directory = self
            .get_directory(dirname)?
            .ok_or(Error::Other("Directory not found".to_string()))?;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let _ = diesel::delete(files.filter(directory_id.eq(directory.id).and(name.eq(filename))))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(())
    }

    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        use super::schema::directories::dsl::*;

        let path = self.check_path(path)?;

        let directory = self
            .get_directory(&path)?
            .ok_or(Error::Other("Directory not found".to_string()))?;

        // I need to check if this directory has children
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let children = directories
            .filter(parent_id.eq(directory.id))
            .select(models::Directory::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        if !children.is_empty() {
            return Err(Error::Other("Directory has other children".to_string()));
        }

        // I need to check if it contains files
        let files_in_dir = self.get_files_in_directory(&directory)?;
        if !files_in_dir.is_empty() {
            return Err(Error::Other("Directory is not empty".to_string()));
        }

        let _ = diesel::delete(directories.filter(super::schema::directories::dsl::id.eq(directory.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    async fn remove_dir_all(&mut self, path: &Utf8Path) -> Result<()> {
        use super::schema::directories::dsl::*;
        let path = self.check_path(path)?;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        // DELETE ON CASCADE will take care of files when their corresponding directory is removed,
        // and DELETE ON CASCADE also handles recursion for the `directories` table (see tests below)
        let _ = diesel::delete(directories.filter(full_path.eq(path.as_str())))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(())
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
