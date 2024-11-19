use diesel::prelude::*;

use crate::types::NumericType;

use super::{Account, Fx, MovementType, Transaction};

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
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
