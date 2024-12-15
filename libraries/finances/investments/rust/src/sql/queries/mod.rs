use diesel::query_builder::SqlQuery;

/// Returns (a query to) all the `MovementNumerable`s for a given account primary-key
pub fn all_movementnumerable_for_account_id() -> SqlQuery {
    // TODO: I can't really do anything with this query, just bind and return.
    // TODO: Maybe this function should take the account_id and the connection,
    // TODO: execute, and return the Vec<MovementNumerable>
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
}

/// Returns (a query to) all the `SnapshotNumerable`s for a given account primary-key
pub fn all_snapshotnumerable_for_account_id() -> SqlQuery {
    // TODO: I can't really do anything with this query, just bind and return.
    // TODO: Maybe this function should take the account_id and the connection,
    // TODO: execute, and return the Vec<SnapshotNumerable>
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
}

/// Returns (a query to) all the `MovementDividend`s for a given account primary-key
pub fn all_movementdividend_for_account_id() -> SqlQuery {
    // TODO: I can't really do anything with this query, just bind and return.
    // TODO: Maybe this function should take the account_id and the connection,
    // TODO: execute, and return the Vec<MovementDividend>
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
}
