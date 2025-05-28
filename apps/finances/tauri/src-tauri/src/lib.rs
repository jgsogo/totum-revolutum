use diesel::r2d2::{ConnectionManager, Pool};
use tauri::Manager;
pub mod commands;
pub mod db;
pub mod errors;
pub mod oxr;
pub use errors::{Error, Result};
use tauri::async_runtime::Mutex;
mod types;
mod views;
use crate::types::ConnectionType;
use diesel::prelude::*;
use finances_accounts::fields::TreeNodeList;
use finances_accounts::models::{Account, AccountHolder, AccountType, Custodian, MovementType, TransactionGroup};
use finances_accounts::sql::filters::accounttype_by_unique_names;
use finances_app_models::{
    google_type, Account as AccountProto, AccountCategory as AccountCategoryProto, AccountType as AccountTypeProto,
    AppState as AppStateProto, Holder as HolderProto, MainContext as MainContextProto,
    MovementType as MovementTypeProto, TransactionGroup as TransactionGroupProto,
};
use openexchangerates::OXRClient;
use oxr::OXRWrapper;
const SAVINGS_UNIQUE_NAMES: &[&str] = &[finances_accounts::constants::accounttype::ASSETS_CURRENT_SAVINGS];
const INVESTMENT_UNIQUE_NAMES: &[&str] = &[
    finances_investments::constants::accounttype::ASSETS_CURRENT_INVESTMENT,
    finances_investments::constants::accounttype::ASSETS_NON_CURRENT_REAL_STATE,
];
const RETIREMENT_UNIQUE_NAMES: &[&str] = &[finances_investments::constants::accounttype::ASSETS_NON_CURRENT_RETIREMENT];

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
    db_pool: Pool<ConnectionManager<ConnectionType>>,
    state: AppStateProto,
    initial_holder_pk: i64,
    oxr_client: Option<OXRClient>,
) -> tauri::App<R> {
    // TODO: See mutability example in the App::manage method. It shows how to update the connection. Of course we don't want here a hardcoded pool. User may want to switch to different DBs

    log::debug!("Get the main context");
    let mut conn = db_pool.get().expect("Get a connection from the Pool");
    let main_context = get_main_context(&mut conn).expect("Error creating main context");
    let holder = AccountHolder::from_pk(initial_holder_pk, &mut conn).expect("Initial holder doesn't exist");

    // TODO: `AppStateProto` should be mutable during application lifetime
    let mut app_state = state.clone();
    app_state.set_holder(HolderProto::new(
        holder.id,
        holder.name,
        holder.is_company,
        holder.photo,
    ));

    // A mutable OXRClient inside a wrapper, so I can cache data returned from server
    let oxr_client = oxr_client.map(|v| OXRWrapper::new(v));

    log::debug!("Crete the tauri app object");
    builder
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout))
                .build(),
        )
        .setup(|app| {
            app.manage(db_pool);
            app.manage(app_state);
            app.manage(main_context);
            app.manage(Mutex::new(oxr_client));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_app_state,
            commands::get_main_context,
            commands::get_holder_context,
            commands::get_account_context,
            // Sending data
            commands::snapshot::create_snapshot,
            commands::transaction::create_transaction,
            commands::transaction::get_transaction,
            //
            commands::last_transactions::past_transactions,
            oxr::commands::get_fx_spot,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

pub fn get_main_context(conn: &mut PgConnection) -> Result<MainContextProto> {
    // All the holders
    let holders: Vec<HolderProto> = AccountHolder::all()
        .select(AccountHolder::as_select())
        .order(finances_accounts::schema::finances_accounts_accountholder::name.asc())
        .load::<AccountHolder>(conn)?
        .into_iter()
        .map(|v| HolderProto::new(v.id, v.name, v.is_company, v.photo))
        .collect();

    log::debug!("Found {} holders", holders.len());

    // All the account types
    let account_types: Vec<AccountTypeProto> = {
        let savings_accounttypes_pks: Vec<i64> = get_accounttypes_for_unique_names(conn, SAVINGS_UNIQUE_NAMES)?;

        let investment_accounttypes_pks: Vec<i64> = get_accounttypes_for_unique_names(conn, INVESTMENT_UNIQUE_NAMES)?;

        let retirement_accounttypes_pks: Vec<i64> = get_accounttypes_for_unique_names(conn, RETIREMENT_UNIQUE_NAMES)?;

        AccountType::all()
            .select(AccountType::as_select())
            // .filter(accounttype_by_unique_names(unique_names))
            .load::<AccountType>(conn)?
            .into_iter()
            .map(|v| {
                let account_category = if savings_accounttypes_pks.contains(&v.id) {
                    AccountCategoryProto::savings()
                } else if investment_accounttypes_pks.contains(&v.id) {
                    AccountCategoryProto::investment()
                } else if retirement_accounttypes_pks.contains(&v.id) {
                    AccountCategoryProto::retirement()
                } else {
                    AccountCategoryProto::other()
                };
                let breadcrumbs = v.get_breadcrumbs(conn)?;
                Ok::<_, Error>(AccountTypeProto::new(v.id, v.name, Some(breadcrumbs), account_category))
            })
            .collect::<Result<Vec<_>>>()?
    };

    // All the movement types
    let movement_types: Vec<MovementTypeProto> = {
        MovementType::all()
            .select(MovementType::as_select())
            // .filter(accounttype_by_unique_names(unique_names))
            .load::<MovementType>(conn)?
            .into_iter()
            .map(|v| {
                let breadcrumbs = v.get_breadcrumbs(conn)?;
                Ok::<_, Error>(MovementTypeProto::new(v.id, v.name, Some(breadcrumbs)))
            })
            .collect::<Result<Vec<_>>>()?
    };

    // All the accounts
    let accounts: Vec<AccountProto> = {
        // Get all accounts (opened and closed). This is important because some movements in old transactions might refer to closed accounts
        let accounts = Account::all()
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
            .load::<(Account, Custodian, AccountType)>(conn)?;

        accounts
            .into_iter()
            .map(|(account, custodian, account_type)| {
                let account_type = account_types
                    .iter()
                    .find(|&acc_type| acc_type.pk() == account_type.id)
                    .ok_or(Error::Other(format!(
                        "Account type 'pk={}' not found in main context",
                        account_type.id
                    )))?;

                let holders_owning_money: Vec<HolderProto> = Account::holders_for_pk(account.id, conn)?
                    .into_iter()
                    .filter(|(_, owns_money)| *owns_money)
                    .map(|(v, _)| HolderProto::new(v.id, v.name, v.is_company, v.photo))
                    .collect();
                let snapshots = commands::get_snapshots(&account, conn)?;
                let last_snapshot = snapshots.first().cloned();

                Ok::<_, Error>(AccountProto::new(
                    account.id,
                    account.name,
                    custodian.into(),
                    account_type.clone(),
                    google_type::CurrencyCode::new(&account.ccy)?,
                    account.identifier,
                    account.description,
                    account.open.into(),
                    account.close.map(|v| v.into()),
                    holders_owning_money,
                    account.is_numerable,
                    last_snapshot,
                ))
            })
            .collect::<Result<Vec<_>>>()?
    };

    // All the transaction groups
    let transaction_groups: Vec<TransactionGroupProto> = //Vec::default();
    {
        TransactionGroup::all()
            .select(TransactionGroup::as_select())
            .load::<TransactionGroup>(conn)?
            .into_iter()
            .map(|v| TransactionGroupProto::new(v.id, v.name, v.description)).collect()
    };

    let context = MainContextProto::new(holders, account_types, movement_types, accounts, transaction_groups);
    Ok(context)
}

/// Given some 'unique_name's, get the PKs for all of them and their descendants
fn get_accounttypes_for_unique_names<'a>(conn: &mut PgConnection, unique_names: &'a [&'a str]) -> Result<Vec<i64>> {
    let account_types = AccountType::all()
        .select((
            finances_accounts::schema::finances_accounts_accounttype::id,
            finances_accounts::schema::finances_accounts_accounttype::tn_descendants_pks,
        ))
        .filter(accounttype_by_unique_names(unique_names))
        .load::<(i64, TreeNodeList)>(conn)?;
    let mut account_types_pks = Vec::default();
    for (acc_type_pk, mut tn_descendants_pks) in account_types {
        account_types_pks.push(acc_type_pk);
        account_types_pks.append(&mut tn_descendants_pks.nodes);
    }
    account_types_pks.sort();
    account_types_pks.dedup();
    Ok(account_types_pks)
}
