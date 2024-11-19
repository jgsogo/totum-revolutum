use diesel::prelude::*;

use crate::types::NumericType;

use super::Account;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_snapshot)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct Snapshot {
    pub id: i64,
    pub amount: NumericType,
    pub date_value: chrono::NaiveDate,
    pub account_id: i64,
}
