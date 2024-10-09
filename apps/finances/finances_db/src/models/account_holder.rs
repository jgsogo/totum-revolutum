use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_accountholder)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct AccountHolder {
    pub id: i32,
    pub name: String,
    pub owner: i32,
}
