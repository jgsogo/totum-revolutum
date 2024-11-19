use diesel::prelude::*;

pub use super::{AccountType, Custodian};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_account)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountType, foreign_key = type_id))]
#[diesel(belongs_to(Custodian, foreign_key = custodian_id))]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub ccy: String,
    pub open: chrono::NaiveDate,
    pub close: Option<chrono::NaiveDate>,
    pub type_id: i64,
    pub custodian_id: i64,
    pub is_numerable: bool,
}

impl Account {
    /// Returns (a query to) all the [`Account`]s together with their [`Custodian`] and [`AccountType`]
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_custodian_and_type() -> _ {
        crate::schema::finances_accounts_account::table
            .inner_join(crate::schema::finances_accounts_custodian::table)
            .inner_join(crate::schema::finances_accounts_accounttype::table)
    }
}
