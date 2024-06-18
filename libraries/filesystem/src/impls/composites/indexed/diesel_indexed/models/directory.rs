use camino::Utf8Path;
use diesel::*;

use crate::impls::composites::FilesystemIndexedDbDirectory;

use super::super::schema::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = directories)]
pub struct Directory {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub full_path: String,
}

#[derive(Insertable)]
#[diesel(table_name = directories)]
pub struct NewDirectory<'a> {
    pub parent_id: Option<i32>,
    pub full_path: &'a str,
}

impl FilesystemIndexedDbDirectory for Directory {
    fn full_path(&self) -> &Utf8Path {
        &Utf8Path::new(&self.full_path)
    }
}
