//! Contains all the Tauri commands that are available to this application.
//!
//! It's useful to have all of them in the same file, because their names (without scope) need
//! to be unique for each Tauri application. We enforce this guarantee if all the commands are
//! defined in the same module.

pub mod account;
pub mod account_list;
pub mod custodian;
pub mod holder;
pub mod movement_type;
pub mod snapshot;
pub mod transaction;
pub mod transaction_group;
use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::fields::TreeNodeList;
use finances_accounts::models::{Account, AccountHolder, AccountHolderRole, AccountType, Custodian};
use finances_accounts::sql::filters::{accountholder_by_pk, accounttype_by_unique_names};
use finances_app_models::{
    Account as AppModelAccount, AccountCategory, AccountType as AppModelAccountType, AppModel, AppState, HolderContext,
    MainContext,
};
use tauri::ipc::Response;
use tauri::State;

trait IntoTauriResponse<R: prost::Message> {
    fn to_response(&self) -> Response;
}

impl<T: AppModel<U>, U: prost::Message> IntoTauriResponse<U> for T {
    fn to_response(&self) -> Response {
        let encoded = self.inner_type_ref().encode_to_vec();
        Response::new(encoded)
    }
}

#[tauri::command]
pub async fn get_app_state(state: State<'_, AppState>) -> Result<Response, String> {
    Ok(state.to_response())
}

#[tauri::command]
pub async fn get_main_context(pool: State<'_, Pool<ConnectionManager<ConnectionType>>>) -> Result<Response, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let holders = AccountHolder::all()
        .select(AccountHolder::as_select())
        .order(finances_accounts::schema::finances_accounts_accountholder::name.asc())
        .load::<AccountHolder>(&mut conn)
        .map_err(|e| format!("Error loading holders: {}", e))?;

    log::debug!("Found {} holders", holders.len());

    // All the account types
    let account_types: Vec<AppModelAccountType> = {
        let savings_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(&mut conn, AccountCategory::savings_accounttypes())?;

        let investment_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(&mut conn, AccountCategory::investment_accounttypes())?;

        let retirement_accounttypes_pks: Vec<i64> =
            get_accounttypes_for_unique_names(&mut conn, AccountCategory::retirement_accounttypes())?;

        AccountType::all()
            .select(AccountType::as_select())
            // .filter(accounttype_by_unique_names(unique_names))
            .load::<AccountType>(&mut conn)
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
                    .get_breadcrumbs(&mut conn)
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

    let context = MainContext::new(holders, account_types);
    Ok(context.to_response())
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

#[tauri::command]
pub async fn get_holder_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    holder_pk: i64,
) -> Result<Response, String> {
    log::info!("Get Holder {holder_pk} context");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let acc_holder = AccountHolder::from_pk(holder_pk, &mut conn).map_err(|e| format!("Error loading holder: {e}"))?;

    let accounts = {
        // Get all accounts for a given holder
        let accounts = Account::all()
            .inner_join(
                finances_accounts::schema::finances_accounts_accountholderrole::table
                    .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
            )
            .inner_join(Custodian::all())
            .inner_join(AccountType::all())
            .filter(accountholder_by_pk(holder_pk))
            .select((
                Account::as_select(),
                AccountHolderRole::as_select(),
                Custodian::as_select(),
                AccountType::as_select(),
            ))
            .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
            .map_err(|e| format!("Error loading accounts: {}", e))?;

        accounts
            .into_iter()
            .map(|(account, account_holder_role, custodian, account_type)| {
                AppModelAccount::new(account, account_holder_role, custodian.into(), account_type.id)
            })
            .collect()
    };

    let context = HolderContext::new(acc_holder, accounts);
    Ok(context.to_response())
}
