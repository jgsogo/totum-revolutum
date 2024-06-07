use crate::diesel::schema::*;
use diesel::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = directories)]
pub struct Directory {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}
