use crate::state::AppState;
use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, NewFx, NewMovement, NewTransaction};
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::managers::create_movement_numerable;
use finances_investments::models::NewMovementNumerable;
use log::info;
use tauri::State;
use thiserror::Error;

#[derive(Debug, Error)]
enum CommandError {
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    DieselError(#[from] diesel::result::Error),
}

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    state: State<'_, AppState>,
    transaction: crate::models::NewTransaction,
) -> Result<usize, String> {
    info!("Create new transaction: {transaction:?}");

    let mut conn = pool.get().expect("Get a connection from the Pool");

    // TODO: Validate amounts, is there a better place?

    conn.transaction(|conn| {
        // Create the transaction
        let transaction_pk = {
            let new_transaction = NewTransaction {
                name: &transaction.name,
                description: transaction.description.as_deref(),
                group_id: transaction.transaction_group_pk.as_ref(),
            };

            diesel::insert_into(finances_accounts::schema::finances_accounts_transaction::table)
                .values(&new_transaction)
                .returning(finances_accounts::schema::finances_accounts_transaction::id)
                .get_result::<i64>(conn)?
        };

        let n_from = create_transaction_movements(
            &state.base_ccy,
            conn,
            &transaction.movements_from,
            0, // FIXME: We want an enum here
            &transaction_pk,
        )?;

        let n_to = create_transaction_movements(
            &state.base_ccy,
            conn,
            &transaction.movements_to,
            1, // FIXME: We want an enum here
            &transaction_pk,
        )?;

        Ok(n_from + n_to)
    })
    .map_err(|e: CommandError| e.to_string())
}

fn create_transaction_movements(
    base_ccy: &str,
    conn: &mut PgConnection,
    movements: &[crate::models::NewMovement],
    direction: i32,
    transaction_pk: &i64,
) -> Result<usize, CommandError> {
    for mov in movements {
        // FIXME: Collect all the accounts instead of doing N queries
        let (account_numerable, account_ccy) = Account::all()
            .select((
                finances_accounts::schema::finances_accounts_account::is_numerable,
                finances_accounts::schema::finances_accounts_account::ccy,
            ))
            .filter(account_by_pk(mov.account_pk))
            .first::<(bool, String)>(conn)?;

        let date_value = chrono::NaiveDate::parse_from_str(&mov.date_value, "%Y-%m-%d")
            .map_err(|e| CommandError::Other(format!("Error parsing date from string ({}): {e}", mov.date_value)))?;

        let fx_id = mov.fx.map_or(Ok::<Option<i64>, CommandError>(None), |fx| {
            let rate: bigdecimal::BigDecimal = fx.try_into().map_err(|e| {
                CommandError::Other(format!("Cannot convert FX rate from f32 ({fx}) to BigDecimal: {e}",))
            })?;
            let new_fx = NewFx {
                foreign: &account_ccy,
                local: base_ccy,
                rate: &rate,
                date_value: &date_value,
            };
            let pk = diesel::insert_into(finances_accounts::schema::finances_accounts_fx::table)
                .values(&new_fx)
                .returning(finances_accounts::schema::finances_accounts_fx::id)
                .get_result::<i64>(conn)?;
            Ok(Some(pk))
        })?;

        let (amount, quantity, unit_value) = mov
            .amount
            .into_bigdecimals(account_numerable)
            .map_err(CommandError::Other)?;

        let new_movement = NewMovement {
            account_id: &mov.account_pk,
            amount: &amount,
            date_value: &date_value,
            direction,
            fx_id: fx_id.as_ref(),
            type_id: &mov.movement_type_pk,
            transaction_id: transaction_pk,
        };

        if account_numerable {
            let new_movement_numerable = NewMovementNumerable {
                new_movement: &new_movement,
                quantity: quantity.as_ref().expect("It has already been tested"),
                unit_value: unit_value.as_ref().expect("It has already been tested"),
            };
            create_movement_numerable(conn, &new_movement_numerable)?;
        } else {
            diesel::insert_into(finances_accounts::schema::finances_accounts_movement::table)
                .values(&new_movement)
                .execute(conn)?;
        }
    }
    Ok(movements.len())
}
