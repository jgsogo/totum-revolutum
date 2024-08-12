use diesel::connection::LoadConnection;
use diesel::*;

use diesel_utils::managers::GetByPkManager;

use crate::impls::composites::{FilesystemIndexedDbDirectory, FilesystemIndexedDbFile};
use crate::{Error, FilePathBuf, Filename, Result};

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
    pub fn filename(&self) -> &Filename {
        // SAFETY. We can assume it's a valid filename as it was validated when created
        // FIXME: We can't assume it's valid
        // FIXME: Better not to execute 'unsafe' here, other's can't use it
        unsafe { Filename::assume_valid(&self.name) }
    }

    #[allow(dead_code)]
    pub fn full_path<Conn: LoadConnection>(&self, conn: &mut Conn) -> Result<FilePathBuf>
    where
        super::Directory: GetByPkManager<i32, Conn, Error = diesel_utils::error::Error>,
    {
        // TODO: It would be much better to "prefetch" the data from FK relations
        let dir = super::Directory::get_by_pk(self.directory_id, conn).map_err(|e| Error::Other(e.to_string()))?;
        Ok(dir.full_path().join_filename(self.filename()))
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = files)]
pub(crate) struct NewFile<'a> {
    pub name: &'a str,
    pub directory_id: i32,
    pub hash: &'a str,
    pub size: i32,
}

impl FilesystemIndexedDbFile for File {
    fn filename(&self) -> &Filename {
        // SAFETY. We can assume it is valid as the DB was populated with valid values
        // FIXME: We can't make this assumption
        // FIXME: We should not use 'unsafe' here, others might want to use the same
        unsafe { Filename::assume_valid(&self.name) }
    }

    fn size(&self) -> u64 {
        self.size as u64
    }

    fn hash(&self) -> &str {
        &self.hash
    }
}
