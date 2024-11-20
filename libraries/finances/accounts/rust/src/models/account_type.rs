use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_accounttype)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(AccountType, foreign_key = tn_parent_id))]
pub struct AccountType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_abstract: bool,
    pub unique_name: Option<String>,

    // FIXME: These are treenode fields, implement them somwhere else if needed
    pub tn_parent_id: Option<i64>,
}

impl AccountType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_accounttype::table
    }

    /// Returns (a query to) all the [`AccountType`]s with 'unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_unique_name() -> _ {
        crate::schema::finances_accounts_accounttype::table
            .filter(crate::schema::finances_accounts_accounttype::unique_name.is_not_null())
    }

    /// Returns (a query to) all the [`AccountType`]s for a given unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_by_unique_name(unique_name: &str) -> _ {
        crate::schema::finances_accounts_accounttype::table
            .filter(crate::schema::finances_accounts_accounttype::unique_name.eq(unique_name))

        // let all_with_unique_name = AccountType::all_with_unique_name();
        // all_with_unique_name.filter(crate::schema::finances_accounts_accounttype::unique_name.eq(Some(unique_name)))
    }
}
