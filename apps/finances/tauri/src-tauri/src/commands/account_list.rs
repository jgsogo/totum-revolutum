use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, AccountHolderRole, AccountType, Custodian, TreeNodeList};
use finances_accounts::sql::filters::{accountholder_by_pk, accounttype_by_unique_names};
use tauri::State;

#[tauri::command]
pub fn get_all_accounts(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
) -> Result<Vec<crate::models::Account>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let accounts = Account::all()
        .inner_join(
            finances_accounts::schema::finances_accounts_accountholderrole::table
                .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
        )
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .select((
            Account::as_select(),
            AccountHolderRole::as_select(),
            Custodian::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>())
}

#[tauri::command]
pub fn get_all_accounts_for_holder(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    holder_pk: i64,
) -> Result<Vec<crate::models::Account>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

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

    Ok(accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>())
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

#[tauri::command]
pub fn get_all_savings_accounts_for_holder(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    holder_pk: i64,
) -> Result<Vec<crate::models::Account>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let savings_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[finances_accounts::constants::accounttype::ASSETS_CURRENT_SAVINGS],
    )?;

    let accounts = Account::all()
        .inner_join(
            finances_accounts::schema::finances_accounts_accountholderrole::table
                .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
        )
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .filter(accountholder_by_pk(holder_pk))
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(savings_accounttypes_pks))
        .select((
            Account::as_select(),
            AccountHolderRole::as_select(),
            Custodian::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>())
}

#[tauri::command]
pub fn get_all_investment_accounts_for_holder(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    holder_pk: i64,
) -> Result<Vec<crate::models::Account>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let investment_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[
            finances_investments::constants::accounttype::ASSETS_CURRENT_INVESTMENT,
            finances_investments::constants::accounttype::ASSETS_NON_CURRENT_REAL_STATE,
        ],
    )?;

    let accounts = Account::all()
        .inner_join(
            finances_accounts::schema::finances_accounts_accountholderrole::table
                .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
        )
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .filter(accountholder_by_pk(holder_pk))
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(investment_accounttypes_pks))
        .select((
            Account::as_select(),
            AccountHolderRole::as_select(),
            Custodian::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>())
}

#[tauri::command]
pub fn get_all_retirement_accounts_for_holder(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    holder_pk: i64,
) -> Result<Vec<crate::models::Account>, String> {
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let retirement_accounttypes_pks: Vec<i64> = get_all_accounttypes(
        &mut conn,
        &[finances_investments::constants::accounttype::ASSETS_NON_CURRENT_RETIREMENT],
    )?;

    let accounts = Account::all()
        .inner_join(
            finances_accounts::schema::finances_accounts_accountholderrole::table
                .inner_join(finances_accounts::schema::finances_accounts_accountholder::table),
        )
        .inner_join(Custodian::all())
        .inner_join(AccountType::all())
        .filter(accountholder_by_pk(holder_pk))
        .filter(finances_accounts::schema::finances_accounts_account::type_id.eq_any(retirement_accounttypes_pks))
        .select((
            Account::as_select(),
            AccountHolderRole::as_select(),
            Custodian::as_select(),
            AccountType::as_select(),
        ))
        .load::<(Account, AccountHolderRole, Custodian, AccountType)>(&mut conn)
        .map_err(|e| format!("Error loading accounts: {}", e))?;

    Ok(accounts
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<crate::models::Account>>())
}
