use diesel::prelude::*;

use crate::types::NumericType;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_fx)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct Fx {
    pub id: i64,
    pub foreign: String,
    pub local: String,
    pub rate: NumericType,
    pub date_value: chrono::NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_accounts_fx)]
pub struct NewFx<'a> {
    pub foreign: &'a str,
    pub local: &'a str,
    pub rate: &'a NumericType,
    pub date_value: &'a chrono::NaiveDate,
}
