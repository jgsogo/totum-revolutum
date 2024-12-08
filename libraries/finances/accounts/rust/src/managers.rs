//! Provides some helper queries that operate on actual instance (diesel is hidden):
//! these functions takes data and return model instances.

use crate::models::{AccountType, MovementType};
use crate::sql::filters::{accounttype_by_pks, movementtype_by_pks};
use diesel::prelude::*;

fn _reorder_breadcrumbs(mut results: Vec<(i64, String)>, order: &[i64]) -> Vec<String> {
    results.sort_by(|lhs, rhs| {
        let lhs_index = order.iter().position(|&x| x == lhs.0);
        let rhs_index = order.iter().position(|&x| x == rhs.0);
        lhs_index.cmp(&rhs_index)
    });
    results.into_iter().map(|v| v.1).collect()
}

pub fn get_breadcrumbs_for_movementtype(
    conn: &mut PgConnection,
    movement_type: &MovementType,
) -> Result<Vec<String>, diesel::result::Error> {
    let me_pk = vec![movement_type.id];
    let all_pks = [movement_type.tn_ancestors_pks.nodes.clone(), me_pk].concat();
    let breadcrumbs = MovementType::all()
        .filter(movementtype_by_pks(&all_pks))
        .select((
            crate::schema::finances_accounts_movementtype::id,
            crate::schema::finances_accounts_movementtype::name,
        ))
        .load::<(i64, String)>(conn)?;

    Ok(_reorder_breadcrumbs(breadcrumbs, &all_pks))
}

pub fn get_breadcrumbs_for_accounttype(
    conn: &mut PgConnection,
    account_type: &AccountType,
) -> Result<Vec<String>, diesel::result::Error> {
    let me_pk = vec![account_type.id];
    let all_pks = [account_type.tn_ancestors_pks.nodes.clone(), me_pk].concat();
    let breadcrumbs = AccountType::all()
        .filter(accounttype_by_pks(&all_pks))
        .select((
            crate::schema::finances_accounts_accounttype::id,
            crate::schema::finances_accounts_accounttype::name,
        ))
        .load::<(i64, String)>(conn)?;

    Ok(_reorder_breadcrumbs(breadcrumbs, &all_pks))
}
