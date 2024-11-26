use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, AccountType, Custodian, TreeNodeList};
use finances_accounts::sql::filters::accounttype_by_unique_names;
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

/// Given some 'unique_name's, get the PKs for all of them and their descendants
fn get_all_accounttypes<'a>(conn: &mut PgConnection, unique_names: &'a [&'a str]) -> Result<Vec<i64>, String> {
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

fn savings_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let savings_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[finances_accounts::constants::accounttype::ASSETS_CURRENT_SAVINGS],
    )?;

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(savings_accounttypes_pks))
        // .filter(Account::mine())
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

fn investment_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let investment_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[
            finances_investments::constants::accounttype::ASSETS_CURRENT_INVESTMENT,
            finances_investments::constants::accounttype::ASSETS_NON_CURRENT_REAL_STATE,
        ],
    )?;

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(investment_accounttypes_pks))
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

fn retirement_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<(Account, Custodian, AccountType)>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let retirement_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[finances_investments::constants::accounttype::ASSETS_NON_CURRENT_RETIREMENT],
    )?;

    Account::all()
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((Account::as_select(), Custodian::as_select(), AccountType::as_select()))
        // .filter(Account::mine())
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(retirement_accounttypes_pks))
        .load::<(Account, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))
}

#[tauri::command]
pub async fn sidebar_menu(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    category: &str,
    holder_pk: i64,
) -> Result<Vec<crate::models::MenuGroup>, String> {
    log::info!("Get Accounts for category {category} and holder {holder_pk}");

    let accounts = if category == "/all" {
        all_accounts(pool)
    } else if category == "/savings" {
        savings_accounts(pool)
    } else if category == "/investments" {
        investment_accounts(pool)
    } else if category == "/retirement" {
        retirement_accounts(pool)
    } else if category == "/other" {
        // TODO: Other accounts not included in the categories above
        Ok(vec![])
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
