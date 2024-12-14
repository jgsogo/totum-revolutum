use crate::state::AppState;
use crate::types::ConnectionType;
use bigdecimal::One;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::fields::MovementDirection;
use finances_accounts::models::{Account, NewFx, NewMovement, NewTransaction};
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::models::NewMovementDividend;
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
        // TODO: Add more checks:
        //  * if 'base_ccy != account_ccy', the fx is required
        //  * ex_dividend_date <= date_value

        // FIXME: Collect all the accounts instead of doing N queries
        let (_, account_ccy) = Account::all()
            .select((
                finances_accounts::schema::finances_accounts_account::is_numerable,
                finances_accounts::schema::finances_accounts_account::ccy,
            ))
            .filter(account_by_pk(mov.account_pk))
            .first::<(bool, String)>(conn)?;

        let date_value = chrono::NaiveDate::parse_from_str(&mov.date_value, "%Y-%m-%d")
            .map_err(|e| CommandError::Other(format!("Error parsing date from string ({}): {e}", mov.date_value)))?;

        let fx_rate: Option<bigdecimal::BigDecimal> = mov
            .fx
            .map(|v| {
                v.try_into().map_err(|e| {
                    CommandError::Other(format!("Cannot convert FX rate from f32 ({v}) to BigDecimal: {e}",))
                })
            })
            .transpose()?;

        let ex_dividend_date: Option<chrono::NaiveDate> = mov
            .ex_dividend_date
            .as_ref()
            .map(|v| {
                chrono::NaiveDate::parse_from_str(v, "%Y-%m-%d")
                    .map_err(|e| CommandError::Other(format!("Error parsing date from string ({v}): {e}")))
            })
            .transpose()?;

        let (amount, quantity, unit_value) = match mov.r#type {
            crate::models::NewMovementType::NonNumerable => {
                mov.amount.into_bigdecimals(false).map_err(CommandError::Other)?
            }
            crate::models::NewMovementType::Numerable => {
                mov.amount.into_bigdecimals(true).map_err(CommandError::Other)?
            }
            crate::models::NewMovementType::Dividend => {
                let unit_value = mov
                    .amount
                    .unit_value
                    .ok_or(CommandError::Other("No unit_value for movement".to_string()))?
                    .try_into()
                    .map_err(|e| {
                        CommandError::Other(format!(
                            "Cannot convert unit_value f32 ({}) to BigDecimal: {e}",
                            mov.amount.unit_value.unwrap()
                        ))
                    })?;
                let snapshot_quantity = {
                    let snapshot_pk = mov
                        .ex_dividend_snapshot_pk
                        .ok_or(CommandError::Other("No snapshot_pk for dividend movmeent".to_string()))?;
                    finances_investments::schema::finances_investments_snapshotnumerable::table
                        .filter(
                            finances_investments::schema::finances_investments_snapshotnumerable::snapshot_ptr_id
                                .eq(snapshot_pk),
                        )
                        .select(finances_investments::schema::finances_investments_snapshotnumerable::quantity)
                        .get_result::<bigdecimal::BigDecimal>(conn)?
                };

                let amount = &unit_value * &snapshot_quantity;
                (amount, Some(snapshot_quantity), Some(unit_value))
            }
        };

        amount_totals += amount.clone() / fx_rate.as_ref().unwrap_or(&bigdecimal::BigDecimal::one());

        // Create the FX
        let fx_id: Option<i64> = fx_rate
            .map(|rate| {
                let new_fx = NewFx {
                    foreign: &account_ccy,
                    local: base_ccy,
                    rate: &rate,
                    date_value: &date_value,
                };
                new_fx.insert_into_db(conn)
            })
            .transpose()?;

        let new_movement = NewMovement {
            account_id: &mov.account_pk,
            amount: &amount,
            date_value: &date_value,
            direction,
            fx_id: fx_id.as_ref(),
            type_id: &mov.movement_type_pk,
            transaction_id: transaction_pk,
        };

        match mov.r#type {
            crate::models::NewMovementType::NonNumerable => new_movement.insert_into_db(conn)?,
            crate::models::NewMovementType::Numerable => {
                let new_movement_numerable = NewMovementNumerable {
                    new_movement: &new_movement,
                    quantity: quantity.as_ref().expect("It has already been tested"),
                    unit_value: unit_value.as_ref().expect("It has already been tested"),
                };
                new_movement_numerable.insert_into_db(conn)?
            }
            crate::models::NewMovementType::Dividend => {
                let new_movement_dividend = NewMovementDividend {
                    new_movement: &new_movement,
                    ex_dividend_date: ex_dividend_date.as_ref().expect("It has already been tested"),
                    unit_value: unit_value.as_ref().expect("It has already been tested"),
                };
                new_movement_dividend.insert_into_db(conn)?
            }
        };
    }
    Ok(amount_totals)
}
