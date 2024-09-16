use diesel::prelude::*;

use super::{AccountHolder, AccountType};
use rust_decimal::Decimal;

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

impl Account {
    pub fn all() -> Vec<Self> {
        todo!("Implement query to return all accounts")
    }

    pub fn all_with_holder_and_type() -> Vec<(Account, AccountHolder, AccountType)> {
        todo!("Implement query to return all accounts with holder and type")
    }
}

impl Account {
    pub fn holder(&self) -> AccountHolder {
        todo!("Return the AccountHolder given an Account")
    }

    pub fn r#type(&self) -> AccountType {
        todo!("Return the AccountType given an Account")
    }

    pub fn position(&self) -> Decimal {
        todo!("Return the position NOW")
    }

    // pub fn get_position(&self, date: Date) -> Decimal {
    //     todo!("Return the position at a given DATE")
    // }
}
