use camino::Utf8PathBuf;
use diesel::connection::LoadConnection;
use diesel::*;

use diesel_utils::managers::GetByPkManager;

use crate::impls::composites::{FilesystemIndexedDbDirectory, FilesystemIndexedDbFile};
use crate::{Error, Result};

use super::super::schema::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = files)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub directory_id: i32,
    pub hash: String,
    pub size: i32,
}

impl File {
    pub fn full_path<Conn: LoadConnection>(&self, conn: &mut Conn) -> Result<Utf8PathBuf>
    where
        super::Directory: GetByPkManager<i32, Conn, Error = diesel_utils::error::Error>,
    {
        let dir = super::Directory::get_by_pk(self.directory_id, conn).map_err(|e| Error::Other(e.to_string()))?;
        Ok(dir.full_path().join(&self.name))
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub directory_id: i32,
    pub hash: &'a str,
    pub size: i32,
}

impl FilesystemIndexedDbFile for File {
    fn filename(&self) -> &str {
        &self.name
    }

    fn size(&self) -> u64 {
        self.size as u64
    }

    fn hash(&self) -> &str {
        &self.hash
    }
}
