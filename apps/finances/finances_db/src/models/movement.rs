use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_movement)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movement {
    pub id: i32,
    pub amount: BigDecimal,
    pub quantity: i32,
    pub unit_value: BigDecimal,
    pub direction: i32,
    pub date: chrono::NaiveDate,
    pub date_value: chrono::NaiveDate,
    pub account_id: i32,
    pub fx_id: i32,
    pub transfer_id: i32,
    pub type_id: i32, // movementtype
}
