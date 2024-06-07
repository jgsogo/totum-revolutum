use crate::diesel::schema::*;
use diesel::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = files)]
pub struct File {
    pub id: i32,
    pub name: Option<String>,
    pub directory_id: i32,
}
