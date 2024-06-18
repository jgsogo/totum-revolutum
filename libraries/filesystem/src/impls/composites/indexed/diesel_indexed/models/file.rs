use super::super::schema::*;
use crate::impls::composites::FilesystemIndexedDbFile;
use diesel::*;

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
