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
