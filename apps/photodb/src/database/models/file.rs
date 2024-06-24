use diesel::connection::LoadConnection;
use diesel::*;

use diesel_utils::managers::GetByPkManager;

use filesystem::impls::composites::{FilesystemIndexedDbDirectory, FilesystemIndexedDbFile};
use filesystem::{Error, FilePathBuf, Filename, Result};

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

    /// The [`pcloud_sdk::types::FileID`] of this file in the PCloud storage
    pub fileid: Option<i64>,

    pub format_id: Option<i32>,

    /// Indicates whether this file has already been processed or not.
    ///
    /// This value is usually true when the files have been added using this application, because
    /// it has run all the processing before uploading them (resize, conversion, metadata,...);
    /// however, when the files are discovered in the remote storage we initialize this flag
    /// as False and some future work need to be done on this file.
    ///
    /// This flag will also be useful if we added some new processing and we want to work on
    /// all the files again. We just need to update this flag to `false` for all the table.
    pub processed: bool,
}

impl File {
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
pub struct NewFile<'a> {
    pub name: &'a str,
    pub directory_id: i32,
    pub hash: &'a str,
    pub size: i32,
    pub fileid: Option<i64>,
    pub format_id: Option<i32>,
    pub processed: bool,
}

impl FilesystemIndexedDbFile for File {
    fn filename(&self) -> &Filename {
        // SAFETY. We assume the database entry satisfies all the [`Filename`] validations.
        unsafe { Filename::assume_valid(&self.name) }
    }

    fn size(&self) -> u64 {
        self.size as u64
    }

    fn hash(&self) -> &str {
        &self.hash
    }
}
