use crate::models::SnapshotNumerable;
use diesel::prelude::*;

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
