use crate::types::ConnectionType;
use crate::{Error, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Transaction as TransactionDb, TransactionGroup as TransactionGroupDb};
use finances_accounts::sql::filters::{
    movement_filter_account_by_pk, movement_filter_by_direction, transaction_by_pk, transactiongroup_by_pk,
};
use finances_app_models::{
    LastTransactionsRequest as LastTransactionsRequestProto, LastTransactionsResponse as LastTransactionsResponseProto,
    MainContext as MainContextProto, MovementDirection, ProtoWrapper, Transaction as TransactionProto,
    TransactionGroup as TransactionGroupProto,
};
use tauri::State;

use super::movement_into_model_movement;

/// Given an account and a direction (as source of target), it returns the last N transactions
/// (removing duplicates) involving that account.
#[tauri::command]
pub async fn past_transactions(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContextProto>,
    request: tauri::ipc::Request<'_>,
) -> Result<tauri::ipc::Response> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err(Error::Other("Error::RequestBodyMustBeRaw".to_string()));
    };

    let transactions_request = LastTransactionsRequestProto::decode(data.to_vec())
        .map_err(|e| Error::Other(format!("Failed to decode data to SnapshotProto: {e}")))?;

    log::info!(
        "Get past transactions for account {} (direction: {})",
        transactions_request.account_pk(),
        transactions_request.account_movement_direction()?
    );

    let mut conn = pool.get().expect("Get a connection from the Pool");

    // Get the last transactions involving the given account and direction
    let mut transactions =
        TransactionDb::all()
            .inner_join(finances_accounts::schema::finances_accounts_movement::table)
            .select(TransactionDb::as_select())
            .filter(movement_filter_account_by_pk(*transactions_request.account_pk()).and(
                movement_filter_by_direction(transactions_request.account_movement_direction()?.into()),
            ))
            .order(finances_accounts::schema::finances_accounts_movement::date_value.desc())
            .limit(20)
            .load::<TransactionDb>(&mut conn)?;

    // Deduplicate transactions
    transactions.sort_by_key(|v| v.name.clone());
    transactions.dedup_by_key(|v| v.name.clone());

    // Get details for all the transactions
    let transactions: Vec<TransactionProto> = transactions
        .into_iter()
        .map(|t| {
            let group = t
                .group_id
                .map(|group_id| {
                    finances_accounts::schema::finances_accounts_transactiongroup::table
                        .filter(transactiongroup_by_pk(group_id))
                        .select(TransactionGroupDb::as_select())
                        .first::<TransactionGroupDb>(&mut conn)
                })
                .transpose()?
                .map(|g| TransactionGroupProto::new(g.id, g.name, g.description));

            let all_movements = finances_investments::sql::queries::all_movements_for_transaction_id(t.id, &mut conn)?
                .into_iter()
                .map(|mov| {
                    let account = main_context.find_account(mov.account_id()).ok_or(Error::Other(format!(
                        "Cannot find account pk '{}' for movement",
                        mov.account_id()
                    )))?;

                    let mov = movement_into_model_movement(mov, account, &main_context, None, &mut conn)?;
                    let direction = mov.direction()?;
                    Ok((mov, direction))
                })
                .collect::<Result<Vec<_>>>()?;
            let (movements_from, movements_to): (Vec<_>, Vec<_>) = all_movements
                .into_iter()
                .partition(|(_, direction)| direction == &MovementDirection::out());

            Ok::<_, Error>(TransactionProto::new(
                Some(t.id),
                t.name,
                t.description,
                group,
                movements_from.into_iter().map(|(mov, _)| mov).collect(),
                movements_to.into_iter().map(|(mov, _)| mov).collect(),
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let response = LastTransactionsResponseProto::new(transactions);
    Ok(tauri::ipc::Response::new(response.encode_to_vec()))
}

pub(crate) fn get_transaction_details(transaction_pk: i64, conn: &mut PgConnection) -> Result<TransactionProto> {
    log::info!("Get Transaction {transaction_pk} details");

    let t = TransactionDb::all()
        .filter(transaction_by_pk(transaction_pk))
        .select(TransactionDb::as_select())
        .first::<TransactionDb>(conn)?;

    let group = t
        .group_id
        .map(|group_id| {
            finances_accounts::schema::finances_accounts_transactiongroup::table
                .filter(transactiongroup_by_pk(group_id))
                .select(TransactionGroupDb::as_select())
                .first::<TransactionGroupDb>(conn)
        })
        .transpose()?
        .map(|g| TransactionGroupProto::new(g.id, g.name, g.description));

    Ok(TransactionProto::new(
        Some(t.id),
        t.name,
        t.description,
        group,
        Vec::new(),
        Vec::new(),
    ))
}
