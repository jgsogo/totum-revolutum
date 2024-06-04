use diesel::prelude::*;

// TODO: Deserialize as FileID
// TODO: Consider using the FILEID as the unique id

#[derive(Insertable)]
#[diesel(table_name = crate::schema::photos)]
pub struct NewPhoto<'a> {
    pub fileid: &'a i64,
    pub path: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::photos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Photo {
    pub id: i32,
    pub fileid: i64,
    pub path: String,
}
