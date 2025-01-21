// use diesel::{query_builder::SqlQuery, RunQueryDsl};
use crate::models::{Movement, MovementDividend, MovementNumerable, SnapshotNumerable};
use diesel::prelude::*;

/// Returns all the [`MovementNumerable`]s for a given account primary-key
pub fn all_movementnumerable_for_account_id<Conn>(
    account_id: i64,
    conn: &mut Conn,
) -> Result<Vec<MovementNumerable>, diesel::result::Error>
where
    Conn: diesel::connection::Connection<Backend = finances_accounts::types::BackendType>
        + diesel::connection::LoadConnection,
{
    diesel::sql_query(
        r#"
        SELECT *
        FROM finances_accounts_movement
        INNER JOIN
            finances_investments_movementnumerable
        ON
            finances_accounts_movement.id = finances_investments_movementnumerable.movement_ptr_id
        WHERE
            finances_accounts_movement.account_id = $1
        ORDER BY
            finances_accounts_movement.date_value DESC
    "#,
    )
    .bind::<diesel::sql_types::Int8, _>(account_id)
    .load::<MovementNumerable>(conn)
}

/// Returns all the [`SnapshotNumerable`]s for a given account primary-key
pub fn all_snapshotnumerable_for_account_id<Conn>(
    account_id: i64,
    conn: &mut Conn,
) -> Result<Vec<SnapshotNumerable>, diesel::result::Error>
where
    Conn: diesel::connection::Connection<Backend = finances_accounts::types::BackendType>
        + diesel::connection::LoadConnection,
{
    diesel::sql_query(
        r#"
        SELECT *
        FROM finances_accounts_snapshot
        INNER JOIN
            finances_investments_snapshotnumerable
        ON
            finances_accounts_snapshot.id = finances_investments_snapshotnumerable.snapshot_ptr_id
        WHERE
            finances_accounts_snapshot.account_id = $1
        ORDER BY
            finances_accounts_snapshot.date_value DESC
    "#,
    )
    .bind::<diesel::sql_types::Int8, _>(account_id)
    .load::<SnapshotNumerable>(conn)
}

/// Returns all the [`MovementDividend`]s for a given account primary-key
pub fn all_movementdividend_for_account_id<Conn>(
    account_id: i64,
    conn: &mut Conn,
) -> Result<Vec<MovementDividend>, diesel::result::Error>
where
    Conn: diesel::connection::Connection<Backend = finances_accounts::types::BackendType>
        + diesel::connection::LoadConnection,
{
    diesel::sql_query(
        r#"
        SELECT *
        FROM finances_accounts_movement
        INNER JOIN
            finances_investments_movementdividend
        ON
            finances_accounts_movement.id = finances_investments_movementdividend.movement_ptr_id
        WHERE
            finances_accounts_movement.account_id = $1
        ORDER BY
            finances_accounts_movement.date_value DESC
    "#,
    )
    .bind::<diesel::sql_types::Int8, _>(account_id)
    .load::<MovementDividend>(conn)
}

/// Returns all the [`MovementDividend`]s for a given account primary-key
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
