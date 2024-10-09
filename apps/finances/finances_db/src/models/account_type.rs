use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_accounttype)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct AccountType {
    pub id: i32,
    pub name: String,
}
