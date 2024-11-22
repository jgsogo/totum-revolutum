use diesel::prelude::*;

use crate::types::NumericType;

use super::Account;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_snapshot)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(Account, foreign_key = account_id))]
pub struct Snapshot {
    pub id: i64,
    pub amount: NumericType,
    pub date_value: chrono::NaiveDate,
    pub account_id: i64,
}

impl Snapshot {
    /// Creates a query to return all the [`Snapshot`] for a given [`Account`]. The snapshots are
    /// in descending order according to their [`Snapshot::date_value`].
    ///
    /// To get the latest snapshot, just take the first from the returned vector.
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_snapshots(account_pk: i64) -> _ {
        crate::schema::finances_accounts_snapshot::table
            .filter(crate::schema::finances_accounts_snapshot::account_id.eq(account_pk))
            .order((crate::schema::finances_accounts_snapshot::date_value.desc(),))
    }
}
