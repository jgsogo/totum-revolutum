use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, AccountType, Custodian};
use finances_accounts::sql::filters::account_is_checking_account;
use tauri::State;

fn all_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

fn checking_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .filter(account_is_checking_account()) // TODO: .filter(account_is_accounttype_or_children(CHECKING_ACCOUNT))
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

fn investment_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .filter(account_is_checking_account()) // TODO: .filter(account_is_accounttype_or_children(INVESTMENT))
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

fn retirement_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .filter(account_is_checking_account()) // TODO: .filter(account_is_accounttype_or_children(RETIREMENT))
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

#[tauri::command]
pub async fn sidebar_menu(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    category: &str,
) -> Result<Vec<crate::models::MenuGroup>, String> {
    log::info!("Get Accounts for category {category}");

    let accounts = if category == "/all" {
        all_accounts(pool)
    } else if category == "/accounts" {
        checking_accounts(pool)
    } else if category == "/investments" {
        investment_accounts(pool)
    } else if category == "/retirement" {
        retirement_accounts(pool)
    } else if category == "/rentals" {
        // TODO: Return links to views about rented properties
        Ok(vec![])
    } else if category == "/taxes" {
        // TODO: Return links to views about taxes: IRPF, 720,...
        Ok(vec![])
    } else {
        log::error!("Unexpected sidebar_menu category '{category}'");
        // FIXME: Return error?
        Err(format!("Unexpected sidebar_menu category '{category}'"))
    };

    match accounts {
        Ok(accounts) => {
            let accounts = accounts
                .into_iter()
                .map(|v| v.into())
                .collect::<Vec<crate::models::Account>>();
            Ok(crate::models::MenuGroup::new_grouped_by_custodian(accounts))
        }
        Err(e) => Err(e),
    }
}
