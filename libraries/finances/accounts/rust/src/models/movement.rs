use diesel::prelude::*;

use crate::types::NumericType;

use super::{Account, Fx, MovementType, Transaction};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::finances_accounts_movement)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(belongs_to(Fx, foreign_key = fx_id))]
#[diesel(belongs_to(MovementType, foreign_key = type_id))]
#[diesel(belongs_to(Transaction, foreign_key = transaction_id))]
pub struct Movement {
    pub id: i64,
    pub amount: NumericType,
    pub direction: i32,
    pub date_value: chrono::NaiveDate,
    pub account_id: i64,
    pub fx_id: Option<i64>,
    pub type_id: i64,
    pub transaction_id: i64,
}

impl Movement {
    /// Returns (a query to) all the ([`Movement`], [`Transaction`], [`MovementType`]) for a given account primary-key
    /// ordered (desc) by date_value
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_related_data(account_pk: i64) -> _ {
        crate::schema::finances_accounts_movement::table
            // .inner_join(crate::schema::finances_accounts_fx::table) // FIXME: I cannot 'inner_join' a nullable FK
            .inner_join(crate::schema::finances_accounts_transaction::table)
            .inner_join(crate::schema::finances_accounts_movementtype::table)
            .filter(crate::schema::finances_accounts_movement::account_id.eq(account_pk))
            .order((crate::schema::finances_accounts_movement::date_value.desc(),))
    }
}
