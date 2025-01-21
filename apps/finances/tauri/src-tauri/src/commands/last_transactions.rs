use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Transaction as TransactionDb, TransactionGroup as TransactionGroupDb};
use finances_accounts::sql::filters::{
    movement_filter_account_by_pk, movement_filter_by_direction, transactiongroup_by_pk,
};
use finances_app_models::{LastTransactionsRequest, LastTransactionsResponse, Transaction};
use finances_app_models::{MainContext, OutgoingModel};
use tauri::State;

use super::movement_into_model_movement;

/// Given an account and a direction (as source of target), it returns the last N transactions
/// (removing duplicates) involving that account.
#[tauri::command]
pub async fn past_transactions(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContext>,
    request: tauri::ipc::Request<'_>,
) -> Result<tauri::ipc::Response, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let transactions_request: LastTransactionsRequest = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to LastTransactionsRequest: {e}"))?;

    log::info!(
        "Get past transactions for account {} (direction: {})",
        transactions_request.account_pk(),
        transactions_request.account_movement_direction()
    );

    let mut conn = pool.get().expect("Get a connection from the Pool");

    // Get the last transactions involving the given account and direction
    let mut transactions = TransactionDb::all()
        .inner_join(finances_accounts::schema::finances_accounts_movement::table)
        .select(TransactionDb::as_select())
        .filter(
            movement_filter_account_by_pk(transactions_request.account_pk()).and(movement_filter_by_direction(
                transactions_request.account_movement_direction().into(),
            )),
        )
        .order(finances_accounts::schema::finances_accounts_movement::date_value.desc())
        .limit(20)
        .load::<TransactionDb>(&mut conn)
        .map_err(|e| format!("Error retrieving transactions from db: {e}"))?;

    // Deduplicate transactions
    transactions.sort_by_key(|v| v.name.clone());
    transactions.dedup_by_key(|v| v.name.clone());

    // Get details for all the transactions
    let transactions: Vec<Transaction> = transactions
        .into_iter()
        .map(|t| {
            let group = t
                .group_id
                .map(|group_id| {
                    finances_accounts::schema::finances_accounts_transactiongroup::table
                        .filter(transactiongroup_by_pk(group_id))
                        .select(TransactionGroupDb::as_select())
                        .first::<TransactionGroupDb>(&mut conn)
                        .map_err(|e| format!("Error retrieving transaction_group from db: {e}"))
                })
                .transpose()?;

            let all_movements = finances_investments::sql::queries::all_movements_for_transaction_id(t.id, &mut conn)
                .map_err(|e| format!("Error retrieving movements from db: {e}"))?
                .into_iter()
                .map(|mov| {
                    let account_ccy = main_context
                        .find_account(mov.account_id())
                        .map(|acc| acc.currency_code.clone())
                        .ok_or(format!("Cannot find account pk '{}' for movement", mov.account_id()))?;

                    movement_into_model_movement(mov, &account_ccy, &main_context, &mut conn)
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok::<_, String>(Transaction::new(t, group, all_movements))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let response = LastTransactionsResponse::new(transactions);
    Ok(tauri::ipc::Response::new(response.encode_to_vec()))
}
