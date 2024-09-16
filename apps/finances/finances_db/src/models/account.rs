use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::data_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub identifier: Option<String>,
    pub name: String,
    pub is_numerable: bool,
    pub ccy: String,
    // open -> Date,
    // close -> Option<Date>,
    pub holder_id: i32,
    pub type_id: i32,
}
