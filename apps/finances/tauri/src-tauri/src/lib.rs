use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
pub mod commands;
pub mod db;
pub mod models;
mod types;
mod views;
use crate::types::ConnectionType;
use diesel::prelude::*;
use finances_accounts::fields::TreeNodeList;
use finances_accounts::models::{AccountHolder, AccountType, MovementType};
use finances_accounts::sql::filters::accounttype_by_unique_names;
use finances_app_models::{
    AccountCategory, AccountType as AppModelAccountType, AppState, MainContext, MovementType as AppModelMovementType,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    db_pool: Pool<ConnectionManager<ConnectionType>>,
    state: AppState,
) -> tauri::App<R> {
    // TODO: See mutability example in the App::manage method. It shows how to update the connection. Of course we don't want here a hardcoded pool. User may want to switch to different DBs

    let mut conn = db_pool.get().expect("Get a connection from the Pool");
    let main_context = get_main_context(&mut conn).expect("Error creating main context");

    builder
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .build(),
        )
        .setup(|app| {
            app.manage(db_pool);
            app.manage(state);
            app.manage(main_context);
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::account_list::get_all_accounts,
            commands::account_list::get_all_accounts_for_holder,
            commands::account_list::get_all_savings_accounts_for_holder,
            commands::account_list::get_all_investment_accounts_for_holder,
            commands::account_list::get_all_retirement_accounts_for_holder,
            commands::account::get_account_details,
            commands::account::get_account_snapshot_latest,
            commands::account::get_account_snapshots,
            commands::account::get_account_movements,
            commands::holder::get_holder_details,
            commands::custodian::get_custodian_details,
            commands::get_app_state,
            commands::get_main_context,
            commands::get_holder_context,
            commands::get_account_context,
            commands::movement_type::get_all_movementtypes,
            commands::movement_type::get_breadcrumbs_for_movementtype,
            commands::transaction_group::get_all_transaction_groups,
            // Sending data
            commands::snapshot::create_snapshot,
            commands::transaction::create_transaction,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

fn get_main_context(conn: &mut PgConnection) -> Result<MainContext, String> {
    let holders = AccountHolder::all()
        .select(AccountHolder::as_select())
        .order(finances_accounts::schema::finances_accounts_accountholder::name.asc())
        .load::<AccountHolder>(conn)
        .map_err(|e| format!("Error loading holders: {}", e))?;

    log::debug!("Found {} holders", holders.len());

    // All the account types
    let account_types: Vec<AppModelAccountType> = {
        let savings_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(conn, AccountCategory::savings_accounttypes())?;

        let investment_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(conn, AccountCategory::investment_accounttypes())?;

        let retirement_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(conn, AccountCategory::retirement_accounttypes())?;

        AccountType::all()
            .select(AccountType::as_select())
            // .filter(accounttype_by_unique_names(unique_names))
            .load::<AccountType>(conn)
            .map_err(|e| format!("Error loading account types: {}", e))?
            .into_iter()
            .map(|v| {
                let account_category = if savings_accounttypes_pks.contains(&v.id) {
                    AccountCategory::Savings
                } else if investment_accounttypes_pks.contains(&v.id) {
                    AccountCategory::Investment
                } else if retirement_accounttypes_pks.contains(&v.id) {
                    AccountCategory::Retirement
                } else {
                    AccountCategory::Other
                };
                let breadcrumbs = v
                    .get_breadcrumbs(conn)
                    .map_err(|e| format!("Error getting breadcrumbs for account type {}: {}", v.id, e))?;
                Ok::<_, String>(AppModelAccountType::new(
                    v.id,
                    v.name,
                    Some(breadcrumbs),
                    account_category,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    // All the movement types
    let movement_types: Vec<AppModelMovementType> = {
        MovementType::all()
            .select(MovementType::as_select())
            // .filter(accounttype_by_unique_names(unique_names))
            .load::<MovementType>(conn)
            .map_err(|e| format!("Error loading account types: {}", e))?
            .into_iter()
            .map(|v| {
                let breadcrumbs = v
                    .get_breadcrumbs(conn)
                    .map_err(|e| format!("Error getting breadcrumbs for movement type {}: {}", v.id, e))?;
                Ok::<_, String>(AppModelMovementType::new(v.id, v.name, Some(breadcrumbs)))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let context = MainContext::new(holders, account_types, movement_types);
    Ok(context)
}

/// Given some 'unique_name's, get the PKs for all of them and their descendants
fn get_accounttypes_for_unique_names<'a>(
    conn: &mut PgConnection,
    unique_names: &'a [&'a str],
) -> Result<Vec<i64>, String> {
    let account_types = AccountType::all()
        .select((
            finances_accounts::schema::finances_accounts_accounttype::id,
            finances_accounts::schema::finances_accounts_accounttype::tn_descendants_pks,
        ))
        .filter(accounttype_by_unique_names(unique_names))
        .load::<(i64, TreeNodeList)>(conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;
    let mut account_types_pks = Vec::default();
    for (acc_type_pk, mut tn_descendants_pks) in account_types {
        account_types_pks.push(acc_type_pk);
        account_types_pks.append(&mut tn_descendants_pks.nodes);
    }
    account_types_pks.sort();
    account_types_pks.dedup();
    Ok(account_types_pks)
}
