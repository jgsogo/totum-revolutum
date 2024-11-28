use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_custodian)]
#[diesel(check_for_backend(crate::types::BackendType))]
pub struct Custodian {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub country: String,
    pub photo: Option<String>,
}

impl Custodian {
    /// Returns (a query to) all the [`Custodian`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_custodian::table
    }
}
