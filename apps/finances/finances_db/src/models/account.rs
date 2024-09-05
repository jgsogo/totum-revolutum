use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::data_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub name: String,
    // pub body: String,
    // pub published: bool,
}
