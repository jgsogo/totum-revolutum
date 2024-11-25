use diesel::backend::Backend;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::prelude::*;
use diesel::sql_types::Text;
use diesel::FromSqlRow;

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
    pub tn_ancestors_pks: TreeNodeList,
    pub tn_ancestors_count: i32,
    pub tn_children_pks: TreeNodeList,
    pub tn_children_count: i32,
    pub tn_descendants_pks: TreeNodeList,
    pub tn_descendants_count: i32,

    pub tn_parent_id: Option<i64>,
}

// #[repr(i32)]
#[derive(Debug, Clone, PartialEq, FromSqlRow)]
pub struct TreeNodeList {
    pub nodes: Vec<i64>,
}

impl<DB> FromSql<Text, DB> for TreeNodeList
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: DB::RawValue<'_>) -> deserialize::Result<Self> {
        let binding = String::from_sql(bytes)?;
        let nodes_str = binding.trim();
        if nodes_str.is_empty() {
            return Ok(TreeNodeList { nodes: Vec::default() });
        }

        let v = nodes_str
            .split(",")
            .map(|it| it.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let treenode = TreeNodeList { nodes: v };
        Ok(treenode)
    }
}

impl AccountType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_accounttype::table
    }
}
