use crate::models::MovementNumerable;
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

/// Returns all the [`MovementNumerable`]s for a given transaction primary-key
pub fn all_movementnumerable_for_transaction_id<Conn>(
    transaction_id: i64,
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
            finances_accounts_movement.transaction_id = $1
        ORDER BY
            finances_accounts_movement.date_value DESC
    "#,
    )
    .bind::<diesel::sql_types::Int8, _>(transaction_id)
    .load::<MovementNumerable>(conn)
}
