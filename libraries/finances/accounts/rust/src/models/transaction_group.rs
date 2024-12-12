use crate::sql::filters::transaction_group_active;
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
    pub end: Option<chrono::NaiveDate>,
}

impl TransactionGroup {
    /// Returns (a query to) all the [`TransactionGroup`]s (only active ones)
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_transactiongroup::table.filter(transaction_group_active())
    }
}
