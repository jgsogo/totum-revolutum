use crate::views::frontend;
use bigdecimal::ToPrimitive;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_db::models::{Account, AccountHolder, AccountType, Snapshot};
use tauri::State;

#[tauri::command]
pub async fn account_detail_command(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    pk: i32,
) -> Result<frontend::Account, String> {
    log::info!("Get Accounts pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let (account, holder, account_type) = Account::get_with_holder_and_type(pk)
        .select((
            Account::as_select(),
            AccountHolder::as_select(),
            AccountType::as_select(),
        ))
        .first::<(Account, AccountHolder, AccountType)>(&mut conn)
        .expect("Error loading accounts");

    Ok(frontend::Account {
        // pk: account.id,
        name: account.name,
        holder: frontend::Holder { name: holder.name },
        r#type: frontend::AccountType {
            name: account_type.name,
        },
        ccy: account.ccy,
        identifier: account.identifier,
    })
}

#[tauri::command]
pub async fn account_snapshot_command(
    pool: State<'_, Pool<ConnectionManager<PgConnection>>>,
    pk: i32,
) -> Result<Option<frontend::Snapshot>, String> {
    log::info!("Get (latest) Snapshot for account pk {pk}");
    let mut conn = pool.get().expect("Get a connection from the Pool");

    let snapshot: Option<Snapshot> = Snapshot::all_snapshots(pk)
        .first(&mut conn)
        .optional()
        .expect("Error returning the last snapshot");
    match snapshot {
        Some(snapshot) => {
            let amount = match snapshot.amount {
                Some(amount) => amount,
                None => snapshot.unit_value.as_ref().unwrap() * snapshot.quantity.as_ref().unwrap(),
            };
            Ok(Some(frontend::Snapshot {
                amount: amount.to_f32().unwrap(),
                quantity: snapshot.quantity,
                unit_value: snapshot.unit_value.and_then(|v| v.to_f32()),
                date_value: snapshot.date_value.format("%Y-%m-%d").to_string(),
            }))
        }
        None => Ok(None),
    }
}
