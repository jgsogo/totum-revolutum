//! Default implementation for [`super::FilesystemIndexedDatabase`] using a database implemented
//! with the `diesel` crate.

use ignore_files::IgnoreFilter;

pub use database::DatabaseImpl;

use crate::impls::composites::FilesystemIndexed;
use crate::{Filesystem, Result};

mod database;
pub mod models;
pub mod schema;

/// Creates a new [`FilesystemIndexed`] using a default implementation of a database (SQLite3)
pub async fn new_filesystem_indexed_with_db<TStorage: Filesystem + 'static>(
    database_url: &str,
    storage: TStorage,
    do_initial_indexing: bool,
    ignore_filter: IgnoreFilter,
) -> Result<FilesystemIndexed<DatabaseImpl, TStorage>> {
    let database = DatabaseImpl::new(database_url)?;
    let fs_indexed = FilesystemIndexed::new(database, storage);
    if do_initial_indexing {
        fs_indexed.initial_sync(ignore_filter).await?;
    }
    Ok(fs_indexed)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ignore_files::IgnoreFilter;

    use crate::impls::composites::indexed::diesel_indexed::database::DatabaseImpl;
    use crate::impls::composites::{new_filesystem_indexed_with_db, FilesystemIndexedDatabase};
    use crate::impls::FilesystemLocalTemp;
    use crate::{DirectoryPathBuf, FilePathBuf, FilenameBuf, Filesystem};

    #[tokio::test]
    async fn test_filesystem_indexed_db() -> anyhow::Result<()> {
        let database_file = tempfile::NamedTempFile::new()?;
        let mut fs = {
            let fs = FilesystemLocalTemp::default();
            new_filesystem_indexed_with_db(
                database_file.path().to_str().unwrap(),
                fs,
                true,
                IgnoreFilter::empty(""),
            )
            .await?
        };

        // Populate the filesystem with some files and directories
        {
            // - a file in the root folder
            let rx = {
                let (mut f, rx) = fs
                    .create(&FilePathBuf::new(
                        DirectoryPathBuf::root(),
                        FilenameBuf::from_str("file.txt").unwrap(),
                    ))
                    .await?;
                f.write_all(b"Some content in the root").await?;
                f.sync_all().await?;
                rx.unwrap()
            };
            let _ = rx.await.unwrap();

            // - a file inside some folder
            let rx = {
                let folder = DirectoryPathBuf::from_str("a/folder").unwrap();
                fs.create_dir_all(&folder).await?;
                let (mut f, rx) = fs
                    .create(&folder.join_filename(FilenameBuf::from_str("file.txt").unwrap()))
                    .await?;
                f.write_all(b"Some other content").await?;
                f.sync_all().await?;
                rx.unwrap()
            };
            let _ = rx.await.unwrap();
        }

        // The filesystem tell us about the files available
        let (tx, rx) = flume::bounded(10);
        fs.walk_directory(tx, IgnoreFilter::empty("")).await?;
        let all_files = rx.try_iter().collect::<Vec<_>>();
        assert_eq!(all_files.len(), 2);

        // Now we can go directly to the database and check it
        let db = DatabaseImpl::new(database_file.path().to_str().unwrap())?;
        let files = db.all_files()?.collect::<Vec<_>>();
        assert_eq!(files.len(), 2);
        let f0 = files.get(0).unwrap();
        let f1 = files.get(1).unwrap();

        let mut conn = db.get_conn()?;
        assert_eq!(f0.full_path(&mut conn)?.as_str(), "file.txt");
        assert_eq!(f1.full_path(&mut conn)?.as_str(), "a/folder/file.txt");

        Ok(())
    }
}
