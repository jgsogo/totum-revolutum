use super::transaction::get_transaction_details;
use crate::types::ConnectionType;
use crate::{Error, Result};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::Transaction as TransactionDb;
use finances_accounts::sql::filters::{movement_filter_account_by_pk, movement_filter_by_direction};
use finances_app_models::{
    LastTransactionsRequest as LastTransactionsRequestProto, LastTransactionsResponse as LastTransactionsResponseProto,
    MainContext as MainContextProto, ProtoWrapper, Transaction as TransactionProto,
};
use tauri::State;

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
        .map(|t| get_transaction_details(&mut conn, &main_context, t))
        .collect::<Result<Vec<_>>>()?;

    let response = LastTransactionsResponseProto::new(transactions);
    Ok(tauri::ipc::Response::new(response.encode_to_vec()))
}
