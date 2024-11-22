use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = crate::schema::finances_accounts_movementtype)]
#[diesel(check_for_backend(crate::types::BackendType))]
#[diesel(belongs_to(MovementType, foreign_key = tn_parent_id))]
pub struct MovementType {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_abstract: bool,
    pub unique_name: Option<String>,

    // FIXME: These are treenode fields, implement them somwhere else if needed
    pub tn_parent_id: Option<i64>,
}

impl MovementType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_movementtype::table
    }
}
