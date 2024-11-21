use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_transactiongroup)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct TransactionGroup {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub cadence: String,
    pub start: chrono::NaiveDate,
}
