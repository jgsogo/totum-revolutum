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

    /// Returns (a query to) all the [`AccountType`]s with 'unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all_with_unique_name() -> _ {
        crate::schema::finances_accounts_movementtype::table
            .filter(crate::schema::finances_accounts_movementtype::unique_name.is_not_null())
    }

    /// Returns (a query to) all the [`MovementType`]s for a given unique_name
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn get_by_unique_name(unique_name: &str) -> _ {
        crate::schema::finances_accounts_movementtype::table
            .filter(crate::schema::finances_accounts_movementtype::unique_name.eq(unique_name))
    }
}

// diesel::table! {
//     finances_accounts_movementtype (id) {
//         id -> Int8,
//         tn_ancestors_pks -> Text,
//         tn_ancestors_count -> Int4,
//         tn_children_pks -> Text,
//         tn_children_count -> Int4,
//         tn_depth -> Int4,
//         tn_descendants_pks -> Text,
//         tn_descendants_count -> Int4,
//         tn_index -> Int4,
//         tn_level -> Int4,
//         tn_priority -> Int4,
//         tn_order -> Int4,
//         tn_siblings_pks -> Text,
//         tn_siblings_count -> Int4,
//         #[max_length = 255]
//         name -> Varchar,
//         description -> Nullable<Text>,
//         is_abstract -> Bool,
//         tn_parent_id -> Nullable<Int8>,
//         #[max_length = 255]
//         unique_name -> Nullable<Varchar>,
//     }
// }
