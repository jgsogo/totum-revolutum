use diesel::prelude::*;

use super::Account;
use bigdecimal::BigDecimal;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::data_snapshot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct Snapshot {
    pub id: i32,
    pub amount: Option<BigDecimal>,
    pub quantity: Option<i32>,
    pub unit_value: Option<BigDecimal>,
    pub date_value: chrono::NaiveDate,
    pub account_id: i32,
}

impl Snapshot {
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_snapshots(account_pk: i32) -> _ {
        crate::schema::data_snapshot::table
            .filter(crate::schema::data_snapshot::account_id.eq(account_pk))
            .order((crate::schema::data_snapshot::date_value.desc(),))
    }
}
