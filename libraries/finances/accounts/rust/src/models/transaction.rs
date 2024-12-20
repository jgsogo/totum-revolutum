use diesel::prelude::*;

use super::TransactionGroup;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::finances_accounts_transaction)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(TransactionGroup, foreign_key = group_id))]
pub struct Transaction {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub group_id: Option<i64>,
}

impl Transaction {
    /// Returns (a query to) all the [`Transaction`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_transaction::table
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::finances_accounts_transaction)]
pub struct NewTransaction<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub group_id: Option<&'a i64>,
}

impl NewTransaction<'_> {
    pub fn insert_into_db(&self, conn: &mut PgConnection) -> Result<i64, diesel::result::Error> {
        diesel::insert_into(crate::schema::finances_accounts_transaction::table)
            .values(self)
            .returning(crate::schema::finances_accounts_transaction::id)
            .get_result::<i64>(conn)
    }
}
