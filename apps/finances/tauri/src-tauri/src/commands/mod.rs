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
use finances_accounts::models::{Account, AccountHolder, AccountHolderRole, AccountType, Custodian};
use finances_accounts::sql::filters::accountholder_by_pk;
use finances_app_models::{Account as AppModelAccount, AccountContext, AppModel, AppState, HolderContext, MainContext};
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
pub async fn get_main_context(main_context: State<'_, MainContext>) -> Result<Response, String> {
    Ok(main_context.to_response())
}

#[tauri::command]
pub async fn get_holder_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContext>,
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
                let account_type = main_context.find_account_type(account_type.id).cloned().ok_or(format!(
                    "Account type 'pk={}' not found in main context",
                    account_type.id
                ))?;
                Ok::<_, String>(AppModelAccount::new(
                    account,
                    account_holder_role,
                    custodian.into(),
                    account_type,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let context = HolderContext::new(acc_holder, accounts);
    Ok(context.to_response())
}

#[tauri::command]
pub async fn get_account_context(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContext>,
    account_pk: i64,
) -> Result<Response, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let account = {
        let (account, account_holder_role, custodian, account_type) =
            Account::details_for_pk(account_pk, &mut conn).map_err(|e| format!("Error loading account: {e}"))?;
        let account_type = main_context.find_account_type(account_type.id).cloned().ok_or(format!(
            "Account type 'pk={}' not found in main context",
            account_type.id
        ))?;
        AppModelAccount::new(account, account_holder_role, custodian.into(), account_type)
    };

    let context = AccountContext::new(account, Vec::default(), Vec::default());
    Ok(context.to_response())
}
