use crate::fields::TreeNodeList;
use crate::sql::filters::accounttype_by_pks;
use crate::utils::reorder_breadcrumbs;
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
    pub tn_ancestors_pks: TreeNodeList,
    pub tn_ancestors_count: i32,
    pub tn_children_pks: TreeNodeList,
    pub tn_children_count: i32,
    pub tn_descendants_pks: TreeNodeList,
    pub tn_descendants_count: i32,

    pub tn_parent_id: Option<i64>,
}

impl AccountType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_accounttype::table
    }
}

impl AccountType {
    pub fn get_breadcrumbs(&self, conn: &mut PgConnection) -> Result<Vec<String>, diesel::result::Error> {
        let me_pk = vec![self.id];
        let all_pks = [self.tn_ancestors_pks.nodes.clone(), me_pk].concat();
        let breadcrumbs = Self::all()
            .filter(accounttype_by_pks(&all_pks))
            .select((
                crate::schema::finances_accounts_accounttype::id,
                crate::schema::finances_accounts_accounttype::name,
            ))
            .load::<(i64, String)>(conn)?;

        Ok(reorder_breadcrumbs(breadcrumbs, &all_pks))
    }
}
