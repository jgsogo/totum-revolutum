use diesel::query_builder::SqlQuery;

/// Returns (a query to) all the `MovementNumerable`s for a given account primary-key
pub fn all_movementnumerable_for_account_id() -> SqlQuery {
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
    "#,
    )
}
