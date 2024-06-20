use camino::Utf8Path;
use diesel::*;

use crate::impls::composites::FilesystemIndexedDbDirectory;
use crate::DirectoryPath;

use super::super::schema::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = directories)]
pub(crate) struct Directory {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub full_path: String,
}

#[derive(Insertable)]
#[diesel(table_name = directories)]
pub(crate) struct NewDirectory<'a> {
    pub parent_id: Option<i32>,
    pub full_path: &'a str,
}

impl FilesystemIndexedDbDirectory for Directory {
    fn full_path(&self) -> &DirectoryPath {
        // SAFETY. We can assume it is valid as it was valid when the database was populated
        // FIXME: We can't really make this assumption.
        // FIXME: We should not use 'unsafe' here, others might want to use the same
        unsafe { DirectoryPath::assume_valid(Utf8Path::new(&self.full_path)) }
    }
}
