use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType};
use tauri::State;

fn all_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn checking_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .filter(Account::checking_account())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn investment_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .filter(Account::investment())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
}

fn retirement_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Vec<(Account, AccountHolder, AccountType)> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    Account::all_with_holder_and_type()
        .filter(Account::opened())
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .filter(Account::mine())
        .filter(Account::retirement())
        .load::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts")
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
        vec![]
    } else if category == "/taxes" {
        // TODO: Return links to views about taxes: IRPF, 720,...
        vec![]
    } else {
        log::error!("Unexpected sidebar_menu category '{category}'");
        // FIXME: Return error?
        vec![]
    };

    let accounts = accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>();

    Ok(crate::models::MenuGroup::new_grouped_by_holder(accounts))
}
