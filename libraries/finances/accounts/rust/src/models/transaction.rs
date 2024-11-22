use diesel::prelude::*;

use super::TransactionGroup;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_transaction)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(TransactionGroup, foreign_key = group_id))]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub group_id: Option<i64>,
}

impl Transaction {
    /// Returns (a query to) all the [`Movement`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_transaction::table
    }
}
