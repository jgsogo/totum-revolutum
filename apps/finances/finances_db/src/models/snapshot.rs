use diesel::prelude::*;

use super::Account;
use crate::types::NumericType;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::data_snapshot)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct Snapshot {
    pub id: i32,
    pub amount: NumericType,
    pub quantity: Option<i32>,
    pub unit_value: Option<NumericType>,
    pub date_value: chrono::NaiveDate,
    pub account_id: i32,
}

impl Snapshot {
    /// Creates a query to return all the [`Snapshot`] for a given [`Account`]. The snapshots are
    /// in descending order according to their [`Snapshot::date_value`].
    ///
    /// To get the latest snapshot, just take the first from the returned vector.
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_snapshots(account_pk: i32) -> _ {
        crate::schema::data_snapshot::table
            .filter(crate::schema::data_snapshot::account_id.eq(account_pk))
            .order((crate::schema::data_snapshot::date_value.desc(),))
    }
}
