use diesel::prelude::*;

use crate::fields::TreeNodeList;
use crate::sql::filters::movementtype_by_pks;
use crate::utils::reorder_breadcrumbs;

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
    pub tn_ancestors_pks: TreeNodeList,
}

impl MovementType {
    /// Returns (a query to) all the [`AccountType`]s
    #[diesel::dsl::auto_type(no_type_alias)]
    pub fn all() -> _ {
        crate::schema::finances_accounts_movementtype::table
    }
}

impl MovementType {
    pub fn get_breadcrumbs(&self, conn: &mut PgConnection) -> Result<Vec<String>, diesel::result::Error> {
        let me_pk = vec![self.id];
        let all_pks = [self.tn_ancestors_pks.nodes.clone(), me_pk].concat();
        let breadcrumbs = Self::all()
            .filter(movementtype_by_pks(&all_pks))
            .select((
                crate::schema::finances_accounts_movementtype::id,
                crate::schema::finances_accounts_movementtype::name,
            ))
            .load::<(i64, String)>(conn)?;

        Ok(reorder_breadcrumbs(breadcrumbs, &all_pks))
    }
}
