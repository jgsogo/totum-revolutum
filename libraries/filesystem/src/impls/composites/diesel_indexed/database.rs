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

use crate::filesystem::FilesystemOps;
use crate::impls::composites::diesel_indexed::models;
use crate::impls::composites::diesel_indexed::models::NewFile;
use crate::{Error, File, FileMetadata, Filesystem, Result};

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

    fn create_file(&self, path: &Utf8Path, hash_value: &str, size_value: i32) -> Result<models::File> {
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let dir = self.get_directory(dirname)?.ok_or(Error::PathDoesNotExist)?;
        let new_file = models::NewFile {
            name: filename,
            directory_id: dir.id,
            hash: hash_value,
            size: size_value,
        };

        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        diesel::insert_into(files)
            .values(&new_file)
            .returning(models::File::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))
    }
}

#[async_trait]
impl Filesystem for Database {
    async fn sync_all(self) -> Result<()> {
        Ok(())
    }

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

        let directory = self.get_directory(dirname)?.ok_or(Error::PathDoesNotExist)?;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let file = files
            .filter(name.eq(filename).and(directory_id.eq(directory.id)))
            .select(models::File::as_select())
            .first(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Error::PathDoesNotExist,
                _ => Error::Other(e.to_string()),
            })?;
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

    async fn remove_file(&mut self, path: &Utf8Path) -> Result<()> {
        use super::schema::files::dsl::*;

        let path = self.check_path(path)?;
        let dirname = path.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let filename = path.file_name().ok_or(Error::Other("Filename expected".to_string()))?;

        let directory = self.get_directory(dirname)?.ok_or(Error::PathDoesNotExist)?;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let _ = diesel::delete(files.filter(directory_id.eq(directory.id).and(name.eq(filename))))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;

        Ok(())
    }

    async fn remove_dir(&mut self, path: &Utf8Path) -> Result<()> {
        use super::schema::directories::dsl::*;

        let path = self.check_path(path)?;

        let directory = self.get_directory(&path)?.ok_or(Error::PathDoesNotExist)?;

        // I need to check if this directory has children
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let children = directories
            .filter(parent_id.eq(directory.id))
            .select(models::Directory::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        if !children.is_empty() {
            return Err(Error::NotEmptyDirectory);
        }

        // I need to check if it contains files
        let files_in_dir = self.get_files_in_directory(&directory)?;
        if !files_in_dir.is_empty() {
            return Err(Error::NotEmptyDirectory);
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

    async fn internal_copy(
        &mut self,
        origin: &Utf8Path,
        target: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        let target_dirname = target.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));
        let target_filename = target
            .file_name()
            .ok_or(Error::Other("Filename expected".to_string()))?;

        // Check that target directory does exist
        let target_dir = self.get_directory(target_dirname)?.ok_or(Error::PathDoesNotExist)?;

        // If not force, check target file doesn't exist
        if !force && self.exists(target).await? {
            return Err(Error::TargetFileExists);
        }

        // Copy row
        let origin_value = self.get_metadata(origin).await?;
        let new_file = NewFile {
            name: target_filename,
            directory_id: target_dir.id,
            hash: &origin_value.hash()?,
            size: origin_value.size()? as i32,
        };

        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        diesel::insert_into(files)
            .values(&new_file)
            .on_conflict((name, directory_id))
            .do_update()
            .set((hash.eq(new_file.hash), size.eq(new_file.size)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(None)
    }
}

#[async_trait]
impl FilesystemOps for Database {
    /// Copies a file from `origin` [`Filesystem`] into `self` [`Filesystem`]. The flag `force` indicates if the
    /// target file should be overridden or not in case it already exists (raises [`Error:TargetFileExists`]).
    async fn copy_from(
        &mut self,
        target: &Utf8Path,
        origin: &dyn Filesystem,
        origin_path: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if self.is_same(origin) {
            self.internal_copy(target, origin_path, force).await
        } else {
            if !force && self.exists(target).await? {
                return Err(Error::TargetFileExists);
            }

            let target = self.check_path(target)?;
            let metadata = origin.get_metadata(origin_path).await?;
            let _ = self.create_file(&target, &metadata.hash()?, metadata.size()? as i32)?;
            Ok(None)
        }
    }

    /// Moves a file from `origin` [`Filesystem`] into `self` [`Filesystem`]. The flag `force` indicates if the
    /// target file should be overridden or not in case it already exists (raises [`Error:TargetFileExists`]).
    async fn move_from(
        &mut self,
        target: &Utf8Path,
        origin: &mut dyn Filesystem,
        origin_path: &Utf8Path,
        force: bool,
    ) -> Result<Option<Receiver<Result<()>>>> {
        if self.is_same(origin) {
            self.internal_move(target, origin_path, force).await
        } else {
            // It doesn't make sense to move a file from another filesystem into this DB
            // implementation because it only stores the metadata, not the file itself.
            Err(Error::Forbidden)
        }
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

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    /// Populates the given database with some data
    async fn populate_db(db: &mut Database) -> anyhow::Result<()> {
        // Some directories
        db.create_dir_all(Utf8Path::new("dir1/subdir1/subsubdir1"))
            .await
            .unwrap();
        db.create_dir_all(Utf8Path::new("dir1/subdir1/subsubdir2"))
            .await
            .unwrap();
        db.create_dir_all(Utf8Path::new("dir1/subdir2")).await.unwrap();
        db.create_dir_all(Utf8Path::new("dir2/subdir1")).await.unwrap();

        // Some files
        db.create_file(Utf8Path::new("dir1/subdir1/subsubdir1/file1.txt"), "file1", 1)?;
        db.create_file(Utf8Path::new("dir1/subdir1/subsubdir1/file2.txt"), "file2", 2)?;
        db.create_file(Utf8Path::new("dir1/subdir1/subsubdir2/file1.txt"), "file1", 3)?;
        db.create_file(Utf8Path::new("dir1/subdir1/subsubdir2/file2.txt"), "file2", 4)?;
        db.create_file(Utf8Path::new("dir1/subdir1/file1.txt"), "file1", 5)?;
        db.create_file(Utf8Path::new("dir1/subdir2/file1.txt"), "file1", 6)?;
        db.create_file(Utf8Path::new("dir1/subdir2/file2.txt"), "file2", 7)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_basic_methods() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let db = Database::new(db_file.path().to_str().unwrap())?;

        let dir_path = Utf8Path::new("dir1");
        let subdir_path = Utf8Path::new("dir1/subdir");
        {
            let dir_not_exists = db.get_directory(dir_path)?;
            assert!(dir_not_exists.is_none());
        }

        let dir_created = {
            let (_, created) = db.get_or_create_directory(dir_path)?;
            assert!(created);

            let (dir_created, created) = db.get_or_create_directory(dir_path)?;
            assert!(!created);
            assert_eq!(dir_created.parent_id, Some(0)); // Root directory is always there

            let dir_get = db.get_directory(dir_path)?;
            assert!(dir_get.is_some());
            assert_eq!(dir_created.id, dir_get.as_ref().unwrap().id);
            assert_eq!(dir_created.full_path, dir_get.as_ref().unwrap().full_path);
            assert_eq!(dir_created.parent_id, dir_get.as_ref().unwrap().parent_id);
            dir_created
        };

        {
            let files = db.get_files_in_directory(&dir_created)?;
            assert!(files.is_empty());
        }

        {
            let (subdir_created, created) = db.get_or_create_directory(subdir_path)?;
            assert!(created);
            assert_eq!(subdir_created.parent_id, Some(dir_created.id));
        };

        {
            let file_created = db.create_file(&subdir_path.join("file1.txt"), "hash", 32)?;
            let subdir = db.get_directory(subdir_path)?.unwrap();
            let files = db.get_files_in_directory(&subdir)?;
            assert_eq!(files.len(), 1);
            assert_eq!(files.get(0), Some(&file_created));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_walk_directory() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await?;

        let (tx, rx) = flume::bounded(100);
        db.walk_directory(tx, 0, Utf8Path::new("")).await.unwrap();

        let all_files = rx.try_iter().collect::<Vec<_>>();
        assert_eq!(all_files.len(), 7);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_metadata() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await?;

        // File found
        let metadata = db
            .get_metadata(Utf8Path::new("dir1/subdir1/subsubdir1/file1.txt"))
            .await?;
        assert_eq!(metadata.path(), "dir1/subdir1/subsubdir1/file1.txt");
        assert_eq!(metadata.hash()?, "file1");
        assert_eq!(metadata.size()?, 1);

        // Directory doesn't exist
        let r = db.get_metadata(Utf8Path::new("not/exists")).await;
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e, Error::PathDoesNotExist), "Assert failed. Error was: {}", e);

        // File is not found in directory
        let r = db.get_metadata(Utf8Path::new("dir1/file-not-found")).await;
        assert!(r.is_err());
        let e = r.unwrap_err();
        assert!(matches!(e, Error::PathDoesNotExist), "Assert failed. Error was: {}", e);
        Ok(())
    }

    #[tokio::test]
    async fn test_exists() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await?;

        let found = db.exists(Utf8Path::new("dir1/subdir1/subsubdir1/file1.txt")).await?;
        assert!(found);

        let not_found = db.exists(Utf8Path::new("dir1/not-found")).await?;
        assert!(!not_found);
        Ok(())
    }

    #[tokio::test]
    async fn test_open() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let db = Database::new(db_file.path().to_str().unwrap())?;
        let r = db.open(Utf8Path::new("anything")).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        let r = db.create(Utf8Path::new("anything")).await;
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dir_all() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;

        let _ = db.get_directory(Utf8Path::new("a"))?;
        let _ = db.get_directory(Utf8Path::new("a/long"))?;
        let _ = db.get_directory(Utf8Path::new("a/long/dir"))?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_file() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;
        let filepath = Utf8Path::new("file1.txt");
        assert!(!db.exists(filepath).await?);
        db.create_file(filepath, "file1", 1)?;
        assert!(db.exists(filepath).await?);
        db.remove_file(filepath).await?;
        assert!(!db.exists(filepath).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_dir() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;

        // A directory without files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        let r = db.remove_dir(Utf8Path::new("a/long/dir")).await;
        assert!(r.is_ok());
        assert!(db.get_directory(Utf8Path::new("a/long/dir"))?.is_none());
        assert!(db.get_directory(Utf8Path::new("a/long"))?.is_some());

        // A directory with files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.create_file(Utf8Path::new("a/long/dir/file.txt"), "file1", 1)?;
        let r = db.remove_dir(Utf8Path::new("a/long/dir")).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::NotEmptyDirectory),
            "Assert failed. Error was: '{}'",
            e
        );

        // A directory with child directories
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        let r = db.remove_dir(Utf8Path::new("a/long")).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::NotEmptyDirectory),
            "Assert failed. Error was: '{}'",
            e
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_remove_dir_all() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;

        // A directory without files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.remove_dir_all(Utf8Path::new("a/long/dir")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir"))?.is_none());
        assert!(db.get_directory(Utf8Path::new("a/long"))?.is_some());

        // A directory with files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.create_file(Utf8Path::new("a/long/dir/file.txt"), "file1", 1)?;
        db.remove_dir_all(Utf8Path::new("a/long/dir")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir"))?.is_none());
        assert!(!db.exists(Utf8Path::new("a/long/dir/file.txt")).await?);
        assert!(db.get_directory(Utf8Path::new("a/long"))?.is_some());

        // A directory with child directories
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.remove_dir_all(Utf8Path::new("a")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir"))?.is_none());
        assert!(db.get_directory(Utf8Path::new("a/long"))?.is_none());
        assert!(db.get_directory(Utf8Path::new("a"))?.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_internal_copy() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = Database::new(db_file.path().to_str().unwrap())?;

        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.create_file(Utf8Path::new("a/long/dir/file.txt"), "file1", 1)?;
        db.create_file(Utf8Path::new("a/long/dir/file2.txt"), "file2", 32)?;

        // Target directory doesn't exist
        let r = db
            .internal_copy(
                Utf8Path::new("a/long/dir/file.txt"),
                Utf8Path::new("another/place.txt"),
                true,
            )
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::PathDoesNotExist),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file already exist (force=false)
        let r = db
            .internal_copy(
                Utf8Path::new("a/long/dir/file.txt"),
                Utf8Path::new("a/long/dir/file2.txt"),
                false,
            )
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(
            matches!(e, Error::TargetFileExists),
            "Assert failed. Error was: '{}'",
            e
        );

        // Target file doesn't exist
        assert!(!db.exists(Utf8Path::new("a/long/dir/file_copy.txt")).await?);
        let r = db
            .internal_copy(
                Utf8Path::new("a/long/dir/file.txt"),
                Utf8Path::new("a/long/dir/file_copy.txt"),
                false,
            )
            .await?;
        assert!(r.is_none()); // Nothing to wait
        assert!(db.exists(Utf8Path::new("a/long/dir/file_copy.txt")).await?);

        // Target file is overridden
        let r = db
            .internal_copy(
                Utf8Path::new("a/long/dir/file.txt"),
                Utf8Path::new("a/long/dir/file2.txt"),
                true,
            )
            .await?;
        assert!(r.is_none()); // Nothing to wait
        let metadata = db.get_metadata(Utf8Path::new("a/long/dir/file2.txt")).await?;
        assert_eq!(metadata.hash()?, "file1");
        assert_eq!(metadata.size()?, 1);

        Ok(())
    }
}
