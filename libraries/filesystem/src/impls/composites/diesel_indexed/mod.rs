//! Default implementation for [`super::FilesystemIndexed`] using a database implemented with
//! with `diesel` crate.

use crate::impls::composites::FilesystemIndexed;
use crate::{FilesystemRead, FilesystemWrite, Result};
use database::Database;
pub(self) mod database;
pub(self) mod models;
mod schema;

// FIXME: Make it just a function returning [`FilesystemIndexed`]
pub struct FilesystemIndexedDB;

impl FilesystemIndexedDB {
    pub async fn new<TStorage: FilesystemRead + FilesystemWrite + 'static>(
        database_url: &str,
        storage: TStorage,
        do_initial_indexing: bool,
    ) -> Result<FilesystemIndexed<Database, TStorage>> {
        let database = Database::new(database_url)?;
        let filesystem_indexed = FilesystemIndexed::new(database, storage);
        if do_initial_indexing {
            filesystem_indexed.sync().await?;
        }
        Ok(filesystem_indexed)
    }
}
