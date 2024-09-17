use diesel::prelude::*;

use super::{AccountHolder, AccountType, Snapshot};
use bigdecimal::BigDecimal;
use diesel::helper_types::{InnerJoin, IntoBoxed};
use diesel::pg::Pg;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::data_account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(AccountHolder, foreign_key = holder_id))]
#[diesel(belongs_to(AccountType, foreign_key = type_id))]
pub struct Account {
    pub id: i32,
    pub identifier: Option<String>,
    pub name: String,
    pub is_numerable: bool,
    pub ccy: String,
    // open -> Date,
    // close -> Option<Date>,
    pub holder_id: i32,
    pub type_id: i32,
}

type QuerySetJoinType<'a> = IntoBoxed<
    'a,
    InnerJoin<
        InnerJoin<crate::schema::data_account::table, crate::schema::data_accountholder::table>,
        crate::schema::data_accounttype::table,
    >,
    Pg,
>;

impl Account {
    // pub fn all() -> Select<crate::schema::data_account::table, AsSelect<Account, Pg>> {
    //     use crate::schema::*;
    //     data_account::table.select(Account::as_select())
    // }

    pub fn all_with_holder_and_type<'a>() -> QuerySetJoinType<'a> {
        use crate::schema::*;
        data_account::table
            .inner_join(data_accountholder::table)
            .inner_join(data_accounttype::table)
            .into_boxed()
    }
}

impl Account {
    pub fn holder(&self) -> AccountHolder {
        todo!("Return the AccountHolder given an Account")
    }

    pub fn r#type(&self) -> AccountType {
        todo!("Return the AccountType given an Account")
    }

    pub fn last_snapshot(&self) -> Option<Snapshot> {
        todo!("Return last snapshot (if any)")
    }

    pub fn position(&self) -> BigDecimal {
        todo!("Return the position NOW")
    }

    // pub fn get_position(&self, date: Date) -> BigDecimal {
    //     todo!("Return the position at a given DATE")
    // }
}
