use crate::types::ConnectionType;
use crate::{Error, Result};
use bigdecimal::BigDecimal;
use bigdecimal::ToPrimitive;
use bigdecimal::Zero;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use finances_app_models::{
    FxQuote as FxQuoteProto, Movement as MovementProto, MovementDirection as MovementDirectionProto,
    Transaction as TransactionProto,
};
use tauri::State;

#[tauri::command]
pub fn create_transaction(
    pool: State<'_, Pool<ConnectionManager<ConnectionType>>>,
    request: tauri::ipc::Request,
) -> std::result::Result<f32, String> {
    let tauri::ipc::InvokeBody::Raw(data) = request.body() else {
        return Err("Error::RequestBodyMustBeRaw".to_string());
    };

    let transaction: TransactionProto = data
        .to_owned()
        .try_into()
        .map_err(|e| format!("Failed to decode data to NewTransaction: {e}"))?;

    let mut conn = pool.get().expect("Get a connection from the Pool");
    insert_into_db(transaction, &mut conn)
        .map(|v| v.to_f32().unwrap())
        .map_err(|e| e.to_string())
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
