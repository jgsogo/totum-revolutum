use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType};
use tauri::State;

#[derive(serde::Serialize, Debug)]
pub struct AccountDetail {
    pub pk: i32,
    pub name: String,
}

#[tauri::command]
pub async fn detail_command(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    pk: i32,
) -> Result<AccountDetail, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let (account, _holder, _account_type) = Account::get_with_holder_and_type(pk)
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .first::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts");

    Ok(AccountDetail {
        pk: account.id,
        name: account.name,
    })
}
