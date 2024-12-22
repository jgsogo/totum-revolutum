use diesel::prelude::*;

use crate::types::NumericType;

use super::Account;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, QueryableByName)]
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
    /// Returns (a query to) all the [`Snapshot`]s (ordered-desc by date_value)
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_snapshot::table
            .order(crate::schema::finances_accounts_snapshot::date_value.desc())
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_accounts_snapshot)]
pub struct NewSnapshot<'a> {
    pub amount: &'a NumericType,
    pub date_value: &'a chrono::NaiveDate,
    pub account_id: &'a i64,
}

impl NewSnapshot<'_> {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
        diesel::insert_into(crate::schema::finances_accounts_snapshot::table)
            .values(self)
            .returning(crate::schema::finances_accounts_snapshot::id)
            .get_result::<i64>(conn)
    }
}
