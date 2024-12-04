//! Provides some helper queries that operate on actual instance (diesel is hidden):
//! these functions takes data and return model instances.

use crate::models::{AccountType, MovementType};
use crate::sql::filters::{accounttype_by_pks, movementtype_by_pks};
use diesel::prelude::*;

pub fn get_breadcrumbs_for_movementtype(
    conn: &mut PgConnection,
    movement_type: &MovementType,
) -> Result<Vec<String>, diesel::result::Error> {
    let me_pk = vec![movement_type.id];
    let all_pks = [movement_type.tn_ancestors_pks.nodes.clone(), me_pk].concat();
    let breadcrumbs = MovementType::all()
        .filter(movementtype_by_pks(&all_pks))
        .select(crate::schema::finances_accounts_movementtype::name)
        .load::<String>(conn)?;
    Ok(breadcrumbs)
}

pub fn get_breadcrumbs_for_accounttype(
    conn: &mut PgConnection,
    account_type: &AccountType,
) -> Result<Vec<String>, diesel::result::Error> {
    let me_pk = vec![account_type.id];
    let all_pks = [account_type.tn_ancestors_pks.nodes.clone(), me_pk].concat();
    let breadcrumbs = AccountType::all()
        .filter(accounttype_by_pks(&all_pks))
        .select(crate::schema::finances_accounts_accounttype::name)
        .load::<String>(conn)?;
    Ok(breadcrumbs)
}
