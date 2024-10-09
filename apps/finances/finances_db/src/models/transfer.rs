use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_transfer)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct Transfer {
    pub id: i32,
    pub description: String,
}
