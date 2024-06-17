use super::super::schema::*;
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

#[derive(Insertable)]
#[diesel(table_name = files)]
pub struct NewFile<'a> {
    pub name: &'a str,
    pub directory_id: i32,
    pub hash: &'a str,
    pub size: i32,
}
