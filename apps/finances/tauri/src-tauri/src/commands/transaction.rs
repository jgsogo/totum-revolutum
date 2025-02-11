use super::movement_into_model_movement;
use crate::types::ConnectionType;
use crate::{Error, Result};
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_accounts::models::{Transaction as TransactionDb, TransactionGroup as TransactionGroupDb};
use finances_accounts::sql::filters::{transaction_by_pk, transactiongroup_by_pk};
use finances_app_models::ProtoWrapper;
use finances_app_models::{
    FxQuote as FxQuoteProto, MainContext as MainContextProto, Movement as MovementProto,
    MovementDirection as MovementDirectionProto, MovementDirection, Transaction as TransactionProto,
    TransactionGroup as TransactionGroupProto,
};
use tauri::ipc::Response;
use tauri::State;

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
) -> Result<f32> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err(Error::Other("Error::RequestBodyMustBeRaw".to_string()));
    };

    let transaction: TransactionProto = data
        .to_owned()
        .try_into()
        .map_err(|e| Error::Other(format!("Failed to decode data to NewTransaction: {e}")))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    insert_into_db(transaction, &mut conn).map(|v| v.to_f32().unwrap())
}

fn insert_into_db(transaction: TransactionProto, conn: &mut PgConnection) -> Result<BigDecimal> {
    let (total_from, total_to) = conn.transaction(|conn| {
        // Create the transaction
        let transaction_pk = {
            let new_transaction = finances_accounts::models::NewTransaction {
                name: transaction.name(),
                description: transaction.description(),
                group_id: transaction.group().map(|g| g.pk()),
            };
            new_transaction.insert_into_db(conn)?
        };

        let total_from: BigDecimal = transaction
            .movements_from()
            .map(|mov| insert_movement_into_db(transaction_pk, mov, MovementDirectionProto::out(), conn))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

        let total_to: BigDecimal = transaction
            .movements_to()
            .map(|mov| insert_movement_into_db(transaction_pk, mov, MovementDirectionProto::r#in(), conn))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .fold(BigDecimal::zero(), |sum, mov_amount| sum + mov_amount);

        Ok::<_, Error>((total_from, total_to))
    })?;

    if !finances_accounts::types::compare_eq(&total_from, &total_to) {
        Err(Error::Other(format!(
            "Mismatched amounts, from {total_from} != to {total_to}"
        )))
    } else {
        Ok(total_from)
    }
}

fn insert_fx_into_db(fx_quote: &FxQuoteProto, conn: &mut PgConnection) -> Result<i64> {
    let rate: BigDecimal = fx_quote.quote()?.try_into()?;
    let new_fx = finances_accounts::models::NewFx {
        foreign: &fx_quote.fx_pair()?.quote().to_string(),
        local: &fx_quote.fx_pair()?.base().to_string(),
        rate: &rate,
        date_value: &fx_quote.date_value()?.try_into()?,
    };
    Ok(new_fx.insert_into_db(conn)?)
}

fn insert_movement_into_db(
    transaction_pk: i64,
    movement: &MovementProto,
    direction: MovementDirectionProto,
    conn: &mut PgConnection,
) -> Result<BigDecimal> {
    // Collect some data
    let movement_amount = movement.movement_amount()?;
    let amount_foreign_ccy = movement_amount.amount()?;

    let date_value: chrono::NaiveDate = movement.date_value()?.try_into()?;
    let fx_id: Option<i64> = movement.fx().map(|quote| insert_fx_into_db(quote, conn)).transpose()?;

    // Create the regular movement
    let new_movement = finances_accounts::models::NewMovement {
        account_id: movement.account_pk(),
        amount: &amount_foreign_ccy.amount(),
        date_value: &date_value,
        direction: direction.into(),
        fx_id: fx_id.as_ref(),
        type_id: &movement.r#type()?.pk(),
        transaction_id: &transaction_pk,
    };

    if let Some(_non_numerable) = movement_amount.as_non_numerable()? {
        new_movement.insert_into_db(conn)?;
    } else if let Some(numerable) = movement_amount.as_numerable()? {
        let quantity: BigDecimal = numerable.quantity()?.try_into()?;
        let unit_value = numerable.unit_value()?.amount();

        let new_numerable = finances_investments::models::NewMovementNumerable {
            new_movement: &new_movement,
            quantity: &quantity,
            unit_value: &unit_value,
        };
        new_numerable.insert_into_db(conn)?;
    } else if let Some(dividend) = movement_amount.as_dividend()? {
        let unit_value = dividend.payout()?.unit_value()?.amount();
        let ex_dividend_date: chrono::NaiveDate = dividend.ex_dividend_date()?.try_into()?;

        let new_dividend = finances_investments::models::NewMovementDividend {
            new_movement: &new_movement,
            ex_dividend_date: &ex_dividend_date,
            unit_value: &unit_value,
        };
        new_dividend.insert_into_db(conn)?;
    } else {
        return Err(Error::Other("MovementAmount type not recognized!".to_string()));
    }

    // Amount in the base currency
    let amount = movement.amount()?;
    Ok(amount.amount())
}

#[tauri::command]
pub async fn get_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    main_context: State<'_, MainContextProto>,
    transaction_pk: i64,
) -> Result<Response> {
    log::info!("Get transaction details (pk: {})", transaction_pk);

    let mut conn = pool.get().expect("Get a connection from the Pool");

    // Get the last transactions involving the given account and direction
    let transaction = TransactionDb::all()
        .inner_join(finances_accounts::schema::finances_accounts_movement::table)
        .select(TransactionDb::as_select())
        .filter(transaction_by_pk(transaction_pk))
        .first::<TransactionDb>(&mut conn)?;

    let transaction = get_transaction_details(&mut conn, &main_context, transaction)?;
    Ok(tauri::ipc::Response::new(transaction.encode_to_vec()))
}

pub(crate) fn get_transaction_details(
    conn: &mut PgConnection,
    main_context: &MainContextProto,
    t: TransactionDb,
) -> Result<TransactionProto> {
    let group = t
        .group_id
        .map(|group_id| {
            finances_accounts::schema::finances_accounts_transactiongroup::table
                .filter(transactiongroup_by_pk(group_id))
                .select(TransactionGroupDb::as_select())
                .first::<TransactionGroupDb>(conn)
        })
        .transpose()?
        .map(|g| TransactionGroupProto::new(g.id, g.name, g.description));

    let all_movements = finances_investments::sql::queries::all_movements_for_transaction_id(t.id, conn)?
        .into_iter()
        .map(|mov| {
            let account = main_context.find_account(mov.account_id()).ok_or(Error::Other(format!(
                "Cannot find account pk '{}' for movement",
                mov.account_id()
            )))?;

            let mov = movement_into_model_movement(mov, account, &main_context, Some(t.id), conn)?;
            let direction = mov.direction()?;
            Ok((mov, direction))
        })
        .collect::<Result<Vec<_>>>()?;
    let (movements_from, movements_to): (Vec<_>, Vec<_>) = all_movements
        .into_iter()
        .partition(|(_, direction)| direction == &MovementDirection::out());

    Ok::<_, Error>(TransactionProto::new(
        Some(t.id),
        t.name,
        t.description,
        group,
        movements_from.into_iter().map(|(mov, _)| mov).collect(),
        movements_to.into_iter().map(|(mov, _)| mov).collect(),
    ))
}
