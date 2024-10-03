use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Selectable, PartialEq, Eq)]
#[diesel(table_name = crate::schema::data_movement)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Movement {
    pub id: i32,
    pub amount: BigDecimal,
    pub quantity: Option<i32>,
    pub unit_value: Option<BigDecimal>,
    pub direction: i32,
    pub date: chrono::NaiveDate,
    pub date_value: chrono::NaiveDate,
    pub account_id: i32,
    pub fx_id: Option<i32>,
    pub transfer_id: i32,
    pub type_id: i32, // movementtype
}

impl Movement {
    /// Returns (a query to) all the `Movement`s for a given account primary-key
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_related_data(account_pk: i32) -> _ {
        crate::schema::data_movement::table
            // .inner_join(crate::schema::data_fx::table) // FIXME: I cannot 'inner_join' a nullable FK
            .inner_join(crate::schema::data_transfer::table)
            .inner_join(crate::schema::data_movementtype::table)
            .filter(crate::schema::data_movement::account_id.eq(account_pk))
            .order((crate::schema::data_movement::date_value.desc(),))
    }
}
