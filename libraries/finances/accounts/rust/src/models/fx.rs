use diesel::prelude::*;

use crate::types::NumericType;

#[derive(Queryable, Selectable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_fx)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct Fx {
    pub id: i64,
    pub foreign: String,
    pub local: String,
    pub rate: NumericType,
    pub date_value: chrono::NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_accounts_fx)]
pub struct NewFx<'a> {
    pub foreign: &'a str,
    pub local: &'a str,
    pub rate: &'a NumericType,
    pub date_value: &'a chrono::NaiveDate,
}

impl NewFx<'_> {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
        diesel::insert_into(crate::schema::finances_accounts_fx::table)
            .values(self)
            .returning(crate::schema::finances_accounts_fx::id)
            .get_result::<i64>(conn)
    }
}
