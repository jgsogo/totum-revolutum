use diesel::prelude::*;

use super::{AccountType, Custodian};
use crate::sql::filters::{account_closed, account_opened};

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
    /// Returns (a query to) all the [`Account`]s (only opened ones)
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_account::table.filter(account_opened())
    }

    /// Returns (a query to) all the [`Account`]s that are closed
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_closed() -> _ {
        crate::schema::finances_accounts_account::table.filter(account_closed())
    }

    /// Returns a query fragment to filter all the [`Account`]s that are opened as of today
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn opened() -> _ {
        crate::schema::finances_accounts_account::close
            .is_null()
            .or(crate::schema::finances_accounts_account::close.ge(diesel::dsl::today))
    }

    // /// Returns a query fragment to filter all the [`Account`]s that are owned by ME
    // #[diesel::dsl::auto_type(no_type_alias)]
    // pub fn mine() -> _ {
    //     crate::schema::data_accountholder::owner.eq(0i32)
    // }
}
