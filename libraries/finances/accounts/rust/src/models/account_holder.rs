use diesel::prelude::*;

use super::Account;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::finances_accounts_accountholder)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct AccountHolder {
    pub id: i64,
    pub name: String,
    pub is_company: bool,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_accountholderrole)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountHolder, foreign_key = holder_id))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct AccountHolderRole {
    pub id: i64,
    pub owns_money: bool,
    pub account_id: i64,
    pub holder_id: i64,
}

impl AccountHolder {
    /// Returns (a query to) all the [`AccountHolder`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_accountholder::table
    }
}
