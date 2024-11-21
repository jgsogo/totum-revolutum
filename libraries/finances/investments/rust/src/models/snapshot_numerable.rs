use diesel::prelude::*;

use finances_accounts::types::NumericType;

use finances_accounts::models::Account;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_investments_snapshotnumerable)]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct SnapshotNumerable {
    pub id: i64,
    pub date_value: chrono::NaiveDate,
    pub quantity: NumericType,
    pub unit_value: NumericType,
    pub account_id: i64,
}
