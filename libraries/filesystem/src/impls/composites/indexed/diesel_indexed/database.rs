use camino::Utf8Path;
use diesel::prelude::*;
use diesel::r2d2::Pool;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use diesel_utils::managers::AllManager;

use crate::impls::composites::indexed::diesel_indexed::models;
use crate::impls::composites::{FilesystemIndexedDatabase, FilesystemIndexedDbFile};
use crate::{Error, Result};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/impls/composites/indexed/diesel_indexed/migrations");
const ROOT_DIRECTORY: &str = "";

/// Implementation of the [`FilesystemIndexedDatabase`] trait using a sqlite3 database and the
/// models defined in [`models::File`] and [`models::Directory`]. To be used out-of-the-box for the
/// [`super::super::FilesystemIndexed`] composite implementation
pub struct DatabaseImpl {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl DatabaseImpl {
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

    pub fn get_conn(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool
            .get()
            .map_err(|e| Error::Other(format!("Failed to get one connection from the pool: {e}")))
    }

    fn create_directory(
        &self,
        dirname: &Utf8Path,
        parent_dir: Option<&models::Directory>,
    ) -> Result<models::Directory> {
        let parent_dirname = dirname.parent().unwrap_or_else(|| Utf8Path::new(ROOT_DIRECTORY));

        let parent_dir_id = match parent_dir {
            None => {
                let parent_dir = self.get_directory(parent_dirname)?;
                parent_dir.id
            }
            Some(p) => {
                if parent_dirname != p.full_path {
                    return Err(Error::Other(
                        "Given parent_dir is not a parent of the directory we are creating".to_string(),
                    ));
                }
                p.id
            }
        };

        use super::schema::directories::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
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

impl FilesystemIndexedDatabase for DatabaseImpl {
    type File = models::File;
    type Directory = models::Directory;

    fn all_files(&self) -> Result<impl Iterator<Item = Self::File>> {
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let all_files =
            models::File::all(models::File::as_select(), &mut conn).map_err(|e| Error::Other(e.to_string()))?;
        let all_files = all_files.collect::<Vec<_>>();
        Ok(all_files.into_iter())
    }

    fn all_directories(&self) -> Result<impl Iterator<Item = Self::Directory>> {
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let all_directories = models::Directory::all(models::Directory::as_select(), &mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        let all_directories = all_directories.collect::<Vec<_>>();
        Ok(all_directories.into_iter())
    }

    fn get_files_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::File>> {
        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let files_in_dir: Vec<models::File> = files
            .filter(directory_id.eq(dir.id))
            .select(models::File::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(files_in_dir.into_iter())
    }

    fn get_directory(&self, path: &Utf8Path) -> Result<Self::Directory> {
        use super::schema::directories::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
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

    fn get_file(&self, dir: &Self::Directory, filename: &str) -> Result<Self::File> {
        use super::schema::files::dsl::*;

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let files_found = files
            .filter(name.eq(filename).and(directory_id.eq(dir.id)))
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
        path: &Utf8Path,
        parent_dir: Option<&Self::Directory>,
    ) -> Result<(Self::Directory, bool)> {
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

    fn delete_file(&self, file: Self::File) -> Result<()> {
        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        diesel::delete(files.filter(id.eq(file.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn get_directories_in_directory(&self, dir: &Self::Directory) -> Result<impl Iterator<Item = Self::Directory>> {
        use super::schema::directories::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        let directories_in_dir: Vec<models::Directory> = directories
            .filter(parent_id.eq(dir.id))
            .select(models::Directory::as_select())
            .load(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(directories_in_dir.into_iter())
    }

    fn delete_directory(&self, dir: Self::Directory) -> Result<()> {
        use super::schema::directories::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        diesel::delete(directories.filter(id.eq(dir.id)))
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn delete_directory_on_cascade(&self, dir: Self::Directory) -> Result<()> {
        // TODO: Detect if DELETE ON CASCADE if activated for this DB
        self.delete_directory(dir)
    }

    fn upsert_file(
        &self,
        dir: &Self::Directory,
        filename: &str,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<()> {
        // Get the size and hash, either from the given value or the existing file
        let (size_, hash_) = match (new_size, new_hash) {
            (Some(size_), Some(hash_)) => (size_, hash_.to_string()),
            _ => match self.get_file(dir, filename) {
                Ok(file) => (
                    new_size.unwrap_or(file.size() as u64),
                    new_hash.unwrap_or(file.hash()).to_string(),
                ),
                Err(e) => match e {
                    Error::PathDoesNotExist => {
                        return Err(Error::Other(
                            "If the target file doesn't exist, both values size and hash need to be provided"
                                .to_string(),
                        ));
                    }
                    _ => {
                        return Err(Error::Other(e.to_string()));
                    }
                },
            },
        };

        let new_file = models::NewFile {
            name: filename,
            directory_id: dir.id,
            hash: &hash_,
            size: size_ as i32,
        };

        // let changes = (hash.eq(hash_), size.eq(size_ as i32));

        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        diesel::insert_into(files)
            .values(&new_file)
            .on_conflict((name, directory_id))
            .do_update()
            .set(&new_file)
            .execute(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(())
    }

    fn update_file(
        &self,
        file: Self::File,
        new_directory: Option<&Self::Directory>,
        new_filename: Option<&str>,
        new_size: Option<u64>,
        new_hash: Option<&str>,
    ) -> Result<Self::File> {
        let new_file = models::NewFile {
            name: new_filename.unwrap_or(file.filename()),
            directory_id: new_directory.map(|v| v.id).unwrap_or(file.directory_id),
            hash: new_hash.unwrap_or(file.hash()),
            size: new_size.unwrap_or(file.size()) as i32,
        };

        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;
        use super::schema::files::dsl::*;
        let file_updated = diesel::update(files)
            .filter(id.eq(file.id))
            .set(&new_file)
            .returning(models::File::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(file_updated)
    }

    fn create_file(&self, dir: &Self::Directory, filename: &str, hash_: &str, size_: i32) -> Result<Self::File> {
        use super::schema::files::dsl::*;
        let mut conn = self.pool.get().map_err(|e| Error::Other(e.to_string()))?;

        let new_file = models::NewFile {
            name: filename,
            directory_id: dir.id,
            hash: hash_,
            size: size_,
        };
        let file = diesel::insert_into(files)
            .values(&new_file)
            .returning(models::File::as_returning())
            .get_result(&mut conn)
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::impls::FilesystemLocalTemp;
    use crate::{Filesystem, FilesystemOps};

    use super::*;

    /// Creates a directory and return it
    async fn create_dir_all(db: &mut DatabaseImpl, path: &Utf8Path) -> models::Directory {
        db.create_dir_all(path).await.unwrap();
        db.get_directory(path).unwrap()
    }

    /// Populates the given database with some data
    async fn populate_db(db: &mut DatabaseImpl) {
        // Some directories
        {
            let dir = create_dir_all(db, Utf8Path::new("dir1/subdir1/subsubdir1")).await;
            db.create_file(&dir, "file1.txt", "file1", 1).unwrap();
            db.create_file(&dir, "file2.txt", "file2", 2).unwrap();
        }
        {
            let dir = create_dir_all(db, Utf8Path::new("dir1/subdir1/subsubdir2")).await;
            db.create_file(&dir, "file1.txt", "file1", 3).unwrap();
            db.create_file(&dir, "file2.txt", "file2", 4).unwrap();
        }
        {
            let dir = db.get_directory(Utf8Path::new("dir1/subdir1")).unwrap();
            db.create_file(&dir, "file1.txt", "file1", 5).unwrap();
        }
        {
            let dir = create_dir_all(db, Utf8Path::new("dir1/subdir2")).await;
            db.create_file(&dir, "file1.txt", "file1", 6).unwrap();
            db.create_file(&dir, "file2.txt", "file2", 7).unwrap();
        }
        {
            let _ = create_dir_all(db, Utf8Path::new("dir2/subdir1")).await;
        }
    }

    #[tokio::test]
    async fn test_walk_directory() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await;

        let (tx, rx) = flume::bounded(100);
        db.walk_directory(tx, 0, Utf8Path::new("")).await.unwrap();

        let all_files = rx.try_iter().collect::<Vec<_>>();
        assert_eq!(all_files.len(), 7);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_metadata() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await;

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
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        populate_db(&mut db).await;

        let found = db.exists(Utf8Path::new("dir1/subdir1/subsubdir1/file1.txt")).await?;
        assert!(found);

        let not_found = db.exists(Utf8Path::new("dir1/not-found")).await?;
        assert!(!not_found);
        Ok(())
    }

    #[tokio::test]
    async fn test_open() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        let r = db.open(Utf8Path::new("anything")).await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        let r = db.create(Utf8Path::new("anything")).await;
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden));
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dir_all() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;

        let _ = db.get_directory(Utf8Path::new("a"))?;
        let _ = db.get_directory(Utf8Path::new("a/long"))?;
        let _ = db.get_directory(Utf8Path::new("a/long/dir"))?;
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_file() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;
        let filepath = Utf8Path::new("file1.txt");
        {
            assert!(!db.exists(filepath).await?);
            let root_dir = db.get_directory(Utf8Path::new(ROOT_DIRECTORY))?;
            db.create_file(&root_dir, "file1.txt", "file1", 1)?;
            assert!(db.exists(filepath).await?);
        }
        db.remove_file(filepath).await?;
        assert!(!db.exists(filepath).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_remove_dir() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        // A directory without files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        let r = db.remove_dir(Utf8Path::new("a/long/dir")).await;
        assert!(r.is_ok());
        assert!(db.get_directory(Utf8Path::new("a/long/dir")).is_err());
        assert!(db.get_directory(Utf8Path::new("a/long")).is_ok());

        // A directory with files
        let dir = create_dir_all(&mut db, Utf8Path::new("a/long/dir")).await;
        db.create_file(&dir, "file.txt", "file1", 1)?;
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
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        // A directory without files
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.remove_dir_all(Utf8Path::new("a/long/dir")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir")).is_err());
        assert!(db.get_directory(Utf8Path::new("a/long")).is_ok());

        // A directory with files
        let dir = create_dir_all(&mut db, Utf8Path::new("a/long/dir")).await;
        db.create_file(&dir, "file.txt", "file1", 1)?;
        db.remove_dir_all(Utf8Path::new("a/long/dir")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir")).is_err());
        assert!(!db.exists(Utf8Path::new("a/long/dir/file.txt")).await?);
        assert!(db.get_directory(Utf8Path::new("a/long")).is_ok());

        // A directory with child directories
        db.create_dir_all(Utf8Path::new("a/long/dir")).await?;
        db.remove_dir_all(Utf8Path::new("a")).await?;
        assert!(db.get_directory(Utf8Path::new("a/long/dir")).is_err());
        assert!(db.get_directory(Utf8Path::new("a/long")).is_err());
        assert!(db.get_directory(Utf8Path::new("a")).is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_internal_copy() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        let dir = create_dir_all(&mut db, Utf8Path::new("a/long/dir")).await;
        db.create_file(&dir, "file.txt", "file1", 1)?;
        db.create_file(&dir, "file2.txt", "file2", 32)?;

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

    #[tokio::test]
    async fn test_internal_move() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        let dir = create_dir_all(&mut db, Utf8Path::new("a/long/dir")).await;
        db.create_file(&dir, "file.txt", "file1", 1)?;
        db.create_file(&dir, "file2.txt", "file2", 32)?;

        // Target directory doesn't exist
        let r = db
            .internal_move(
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
            .internal_move(
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
            .internal_move(
                Utf8Path::new("a/long/dir/file.txt"),
                Utf8Path::new("a/long/dir/file_copy.txt"),
                false,
            )
            .await?;
        assert!(r.is_none()); // Nothing to wait
        assert!(db.exists(Utf8Path::new("a/long/dir/file_copy.txt")).await?);
        assert!(!db.exists(Utf8Path::new("a/long/dir/file.txt")).await?);

        // Target file is overridden
        let r = db
            .internal_move(
                Utf8Path::new("a/long/dir/file_copy.txt"),
                Utf8Path::new("a/long/dir/file2.txt"),
                true,
            )
            .await?;
        assert!(r.is_none()); // Nothing to wait
        let metadata = db.get_metadata(Utf8Path::new("a/long/dir/file2.txt")).await?;
        assert_eq!(metadata.hash()?, "file1");
        assert_eq!(metadata.size()?, 1);
        assert!(!db.exists(Utf8Path::new("a/long/dir/file_copy.txt")).await?);

        Ok(())
    }

    #[tokio::test]
    async fn test_copy_from() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        // // If it's within the DB filesystem, it's just a `copy_internal`:
        // TODO: I can't get mutable and inmutable borrow at the same time
        // db.copy_from(Utf8Path::new("copy.txt"), &db, Utf8Path::new("file.txt"), true).await?;
        // assert!(db.exists(Utf8Path::new("copy.txt")).await?);

        let mut temp_fs = FilesystemLocalTemp::default();
        let (_, _) = temp_fs.create(Utf8Path::new("file.txt")).await?;

        db.copy_from(Utf8Path::new("copy.txt"), &temp_fs, Utf8Path::new("file.txt"), false)
            .await?;
        assert!(db.exists(Utf8Path::new("copy.txt")).await?);
        assert!(!db.exists(Utf8Path::new("file.txt")).await?);
        Ok(())
    }

    #[tokio::test]
    async fn test_move_from() -> anyhow::Result<()> {
        let db_file = NamedTempFile::new()?;
        let mut db = DatabaseImpl::new(db_file.path().to_str().unwrap())?;

        // // If it's within the DB filesystem, it's just a `copy_internal`:
        // TODO: I can't get mutable and inmutable borrow at the same time
        // db.move_from(Utf8Path::new("copy.txt"), &db, Utf8Path::new("file.txt"), true).await?;
        // assert!(db.exists(Utf8Path::new("copy.txt")).await?);

        let mut temp_fs = FilesystemLocalTemp::default();
        let (_, _) = temp_fs.create(Utf8Path::new("file.txt")).await?;

        let r = db
            .move_from(
                Utf8Path::new("copy.txt"),
                &mut temp_fs,
                Utf8Path::new("file.txt"),
                false,
            )
            .await;
        assert!(r.is_err());
        let Err(e) = r else { unreachable!() };
        assert!(matches!(e, Error::Forbidden), "Assert failed. Error was: {}", e);
        assert!(!db.exists(Utf8Path::new("copy.txt")).await?);
        assert!(temp_fs.exists(Utf8Path::new("file.txt")).await?);
        Ok(())
    }
}
