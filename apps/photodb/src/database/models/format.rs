use diesel::*;

use super::super::schema::*;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable, AsChangeset, QueryableByName, Selectable,
)]
#[diesel(table_name = formats)]
pub struct Format {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub format: String,
}

#[derive(Insertable)]
#[diesel(table_name = formats)]
pub struct NewFormat<'a> {
    pub parent_id: Option<i32>,
    pub format: &'a str,
}
