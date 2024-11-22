use diesel::prelude::*;

use finances_accounts::types::NumericType;

use finances_accounts::models::Movement;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_investments_movementdividend)]
#[diesel(primary_key(movement_ptr_id))]
#[diesel(check_for_backend(finances_accounts::types::BackendType))]
#[diesel(belongs_to(Movement, foreign_key = movement_ptr_id))]
pub struct MovementDividend {
    pub movement_ptr_id: i64,
    pub ex_dividend_date: chrono::NaiveDate,
    pub unit_value: NumericType,
}
