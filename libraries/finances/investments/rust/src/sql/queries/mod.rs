use crate::models::Movement;
use diesel::prelude::*;

mod movement_dividend;
mod movement_numerable;
mod snapshot_numerable;
pub use movement_dividend::{all_movementdividend_for_account_id, all_movementdividend_for_transaction_id};
pub use movement_numerable::{all_movementnumerable_for_account_id, all_movementnumerable_for_transaction_id};
pub use snapshot_numerable::all_snapshotnumerable_for_account_id;

/// Returns all the movements for a given account primary-key. The objects are instances of [`Movement`], an enum type
/// that holds all possible movement variantes: dividend, numerable,...
pub fn all_movements_for_account_id<Conn>(
    account_id: i64,
    conn: &mut Conn,
) -> Result<Vec<Movement>, diesel::result::Error>
where
    Conn: diesel::connection::Connection<Backend = finances_accounts::types::BackendType>
        + diesel::connection::LoadConnection,
{
    let numerable_movs = all_movementnumerable_for_account_id(account_id, conn)?;
    let dividend_movs = all_movementdividend_for_account_id(account_id, conn)?;
    let regular_movs: Vec<finances_accounts::models::Movement> = {
        let seen_pks: Vec<i64> = {
            [
                numerable_movs.iter().map(|v| v.movement.id).collect::<Vec<_>>(),
                dividend_movs.iter().map(|v| v.movement.id).collect(),
            ]
            .concat()
        };

        finances_accounts::models::Movement::all()
            .select(finances_accounts::models::Movement::as_select())
            .filter(
                finances_accounts::sql::filters::movement_filter_account_by_pk(account_id).and(diesel::dsl::not(
                    finances_accounts::schema::finances_accounts_movement::id.eq_any(seen_pks),
                )),
            )
            .load(conn)?
    };

    let mut r = Vec::default();
    r.extend(numerable_movs.into_iter().map(Movement::Numerable));
    r.extend(dividend_movs.into_iter().map(Movement::Dividend));
    r.extend(regular_movs.into_iter().map(Movement::NonNumerable));
    r.sort_by(|a, b| b.date_value().cmp(a.date_value()));
    Ok(r)
}

/// Returns all the [`MovementDividend`]s for a given transaction primary-key
pub fn all_movements_for_transaction_id<Conn>(
    transaction_id: i64,
    conn: &mut Conn,
) -> Result<Vec<Movement>, diesel::result::Error>
where
    Conn: diesel::connection::Connection<Backend = finances_accounts::types::BackendType>
        + diesel::connection::LoadConnection,
{
    let numerable_movs = all_movementnumerable_for_transaction_id(transaction_id, conn)?;
    let dividend_movs = all_movementdividend_for_transaction_id(transaction_id, conn)?;
    let regular_movs: Vec<finances_accounts::models::Movement> = {
        let seen_pks: Vec<i64> = {
            [
                numerable_movs.iter().map(|v| v.movement.id).collect::<Vec<_>>(),
                dividend_movs.iter().map(|v| v.movement.id).collect(),
            ]
            .concat()
        };

        finances_accounts::models::Movement::all()
            .select(finances_accounts::models::Movement::as_select())
            .filter(
                finances_accounts::sql::filters::movement_filter_transaction_by_pk(transaction_id).and(
                    diesel::dsl::not(finances_accounts::schema::finances_accounts_movement::id.eq_any(seen_pks)),
                ),
            )
            .load(conn)?
    };

    let mut r = Vec::default();
    r.extend(numerable_movs.into_iter().map(Movement::Numerable));
    r.extend(dividend_movs.into_iter().map(Movement::Dividend));
    r.extend(regular_movs.into_iter().map(Movement::NonNumerable));
    r.sort_by(|a, b| b.date_value().cmp(a.date_value()));
    Ok(r)
}
