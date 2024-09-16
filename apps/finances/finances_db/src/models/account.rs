use diesel::prelude::*;

use super::{AccountHolder, AccountType};

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
