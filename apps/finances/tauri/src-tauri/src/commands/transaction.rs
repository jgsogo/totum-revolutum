use crate::types::ConnectionType;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Account, NewMovement, NewTransaction};
use finances_accounts::sql::filters::account_by_pk;
use finances_investments::managers::create_movement_numerable;
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
    transaction: crate::models::NewTransaction,
) -> Result<usize, String> {
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

            diesel::insert_into(finances_accounts::schema::finances_accounts_transaction::table)
                .values(&new_transaction)
                .returning(finances_accounts::schema::finances_accounts_transaction::id)
                .get_result::<i64>(conn)?
        };

        // Create the movements from
        let n_from = create_transaction_movements(
            conn,
            &transaction.movements_from,
            0, // FIXME: We want an enum here
            transaction.date_value.as_ref(),
            &transaction_pk,
        )?;

        let n_to = create_transaction_movements(
            conn,
            &transaction.movements_to,
            1, // FIXME: We want an enum here
            transaction.date_value.as_ref(),
            &transaction_pk,
        )?;

        Ok(n_from + n_to)
    })
    .map_err(|e: CommandError| e.to_string())
}

fn create_transaction_movements(
    conn: &mut PgConnection,
    movements: &[crate::models::NewMovement],
    direction: i32,
    transaction_date_value: Option<&String>,
    transaction_pk: &i64,
) -> Result<usize, CommandError> {
    for movement_from in movements {
        // FIXME: Collect all the accounts instead of doing N queries
        let account_numerable = Account::all()
            .select(finances_accounts::schema::finances_accounts_account::is_numerable)
            .filter(account_by_pk(movement_from.account_pk))
            .first::<bool>(conn)?;

        let date_value = {
            let date_str = movement_from
                .date_value
                .as_ref()
                .or(transaction_date_value)
                .ok_or(CommandError::Other("Movement without date".to_string()))?;

            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .map_err(|e| CommandError::Other(format!("Error parsing date from string ({}): {e}", date_str)))?
        };
        let fx_id = movement_from.fx.map(|_fx| 0i64); // FIXME: Create the fx and return pk

        if account_numerable {
            let quantity: bigdecimal::BigDecimal = movement_from
                .quantity
                .ok_or(CommandError::Other("No qunatity for movement".to_string()))?
                .try_into()
                .map_err(|e| {
                    CommandError::Other(format!(
                        "Cannot convert quantity f32 ({}) to BigDecimal: {e}",
                        movement_from.quantity.unwrap()
                    ))
                })?;

            let unit_value: bigdecimal::BigDecimal = movement_from
                .unit_value
                .ok_or(CommandError::Other("No unit_value for movement".to_string()))?
                .try_into()
                .map_err(|e| {
                    CommandError::Other(format!(
                        "Cannot convert unit_value f32 ({}) to BigDecimal: {e}",
                        movement_from.unit_value.unwrap()
                    ))
                })?;

            let amount: bigdecimal::BigDecimal = &quantity * &unit_value;

            create_movement_numerable(
                conn,
                &amount,
                direction,
                &date_value,
                &movement_from.account_pk,
                fx_id.as_ref(),
                &movement_from.movement_type_pk,
                transaction_pk,
                &quantity,
                &unit_value,
            )?;
        } else {
            let amount: bigdecimal::BigDecimal = movement_from
                .amount
                .ok_or(CommandError::Other("No amount for movement".to_string()))?
                .try_into()
                .map_err(|e| {
                    CommandError::Other(format!(
                        "Cannot convert amount f32 ({}) to BigDecimal: {e}",
                        movement_from.amount.unwrap()
                    ))
                })?;

            let new_movement = NewMovement {
                account_id: &movement_from.account_pk,
                amount: &amount,
                date_value: &date_value,
                direction,
                fx_id: fx_id.as_ref(),
                type_id: &movement_from.movement_type_pk,
                transaction_id: transaction_pk,
            };

            diesel::insert_into(finances_accounts::schema::finances_accounts_movement::table)
                .values(&new_movement)
                .execute(conn)?;
        }
    }
    Ok(0) // FIXME: Return the number of created movements
}
