use crate::state::AppState;
use crate::types::ConnectionType;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::fields::MovementDirection;
use finances_accounts::models::{Account, NewFx, NewMovement, NewTransaction};
use finances_accounts::sql::filters::account_by_pk;
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
) -> Result<f32, String> {
    info!("Create new transaction: {transaction:?}");

    let mut conn = pool.get().expect("Get a connection from the Pool");

    conn.transaction(|conn| {
        // Create the transaction
        let transaction_pk = {
            let new_transaction = NewTransaction {
                name: &transaction.name,
                description: transaction.description.as_deref(),
                group_id: transaction.transaction_group_pk.as_ref(),
            };
            new_transaction.insert_into_db(conn)?
        };

        let total_from = create_transaction_movements(
            &state.base_ccy,
            conn,
            &transaction.movements_from,
            MovementDirection::Out,
            &transaction_pk,
        )?;

        let total_to = create_transaction_movements(
            &state.base_ccy,
            conn,
            &transaction.movements_to,
            MovementDirection::In,
            &transaction_pk,
        )?;

        if total_from != total_to {
            Err(CommandError::Other(format!(
                "Mismatched amounts, from {total_from} != to {total_to}"
            )))
        } else {
            Ok(total_from
                .to_f32()
                .ok_or(CommandError::Other(format!("Cannot convert {total_from} back to f32")))?)
        }
    })
    .map_err(|e: CommandError| e.to_string())
}

fn create_transaction_movements(
    base_ccy: &str,
    conn: &mut PgConnection,
    movements: &[crate::models::NewMovement],
    direction: MovementDirection,
    transaction_pk: &i64,
) -> Result<bigdecimal::BigDecimal, CommandError> {
    let mut amount_totals = bigdecimal::BigDecimal::zero();

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

        // TODO: Now I need to match the movement type
        // todo!("Match movement type");
        let (amount, quantity, unit_value) = mov
            .amount
            .into_bigdecimals(account_numerable)
            .map_err(CommandError::Other)?;

        let (fx_id, amount_base_ccy) = mov.fx.map_or(
            Ok::<(Option<i64>, bigdecimal::BigDecimal), CommandError>((None, amount.clone())),
            |fx| {
                let rate: bigdecimal::BigDecimal = fx.try_into().map_err(|e| {
                    CommandError::Other(format!("Cannot convert FX rate from f32 ({fx}) to BigDecimal: {e}",))
                })?;
                let new_fx = NewFx {
                    foreign: &account_ccy,
                    local: base_ccy,
                    rate: &rate,
                    date_value: &date_value,
                };
                let pk = new_fx.insert_into_db(conn)?;
                Ok((Some(pk), amount.clone() / rate))
            },
        )?;

        amount_totals += amount_base_ccy;

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
            new_movement_numerable.insert_into_db(conn)?;
        } else {
            new_movement.insert_into_db(conn)?;
        }
    }
    Ok(amount_totals)
}
