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

    // FIXME: These are treenode fields, implement them somwhere else if needed
    pub tn_parent_id: Option<i64>,
}
