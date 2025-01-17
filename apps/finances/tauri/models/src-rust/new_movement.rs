use crate::google_type;
use bigdecimal::{BigDecimal, One};
use finances_accounts::fields::MovementDirection;
use finances_accounts::models::{NewFx as NewFxDb, NewMovement as NewMovementDb};
use finances_investments::models::{
    NewMovementDividend as NewMovementDividendDb, NewMovementNumerable as NewMovementNumerableDb,
};

use diesel::prelude::*;

pub struct NewMovement {
    r#type: NewMovementAmount,
    account_pk: i64,
    date_value: chrono::NaiveDate,
    movement_type_pk: i64,
    fx: Option<BigDecimal>,
}

impl NewMovement {
    /// Inserts the transaction into the database and returns the amount-sum of all the movements (in one direction)
    pub fn insert_into_db(
        &self,
        conn: &mut PgConnection,
        transaction_pk: i64,
        direction: MovementDirection,
        base_ccy: &str,
    ) -> Result<BigDecimal, diesel::result::Error> {
        let (amount, ccy) = self.r#type.get_amount();

        // Create the FX
        let fx_id: Option<i64> = self
            .fx
            .as_ref()
            .map(|rate| {
                let new_fx = NewFxDb {
                    foreign: ccy,
                    local: base_ccy,
                    rate,
                    date_value: &self.date_value,
                };
                new_fx.insert_into_db(conn)
            })
            .transpose()?;

        let new_movement = NewMovementDb {
            account_id: &self.account_pk,
            amount: &amount,
            date_value: &self.date_value,
            direction,
            fx_id: fx_id.as_ref(),
            type_id: &self.movement_type_pk,
            transaction_id: &transaction_pk,
        };

        match &self.r#type {
            NewMovementAmount::NonNumerable(_) => new_movement.insert_into_db(conn)?,
            NewMovementAmount::Numerable(numerable) => {
                let new_movement_numerable = NewMovementNumerableDb {
                    new_movement: &new_movement,
                    quantity: &numerable.quantity,
                    unit_value: &numerable.unit_value,
                };
                new_movement_numerable.insert_into_db(conn)?
            }
            NewMovementAmount::DividendAmount(dividend) => {
                let new_movement_dividend = NewMovementDividendDb {
                    new_movement: &new_movement,
                    ex_dividend_date: &dividend.ex_dividend_date,
                    unit_value: &dividend.payout.unit_value,
                };
                new_movement_dividend.insert_into_db(conn)?
            }
        };

        let amount_in_base_ccy = amount / self.fx.as_ref().unwrap_or(&bigdecimal::BigDecimal::one());
        Ok(amount_in_base_ccy)
    }
}

// TODO: Duplicated in the 'new_snapshot' module
struct NewNumerableAmount {
    quantity: bigdecimal::BigDecimal,
    unit_value: bigdecimal::BigDecimal,
    ccy: String,
}

impl NewNumerableAmount {
    pub fn get_amount(&self) -> (BigDecimal, &str) {
        (&self.quantity * &self.unit_value, &self.ccy)
    }
}

impl TryFrom<crate::protos::finances_app_models::money_amount::Numerable> for NewNumerableAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::money_amount::Numerable) -> Result<Self, Self::Error> {
        let unit_value = google_type::Money(v.unit_value.unwrap());
        let (unit_value, ccy): (BigDecimal, String) = unit_value.into();
        let quantity = google_type::Decimal(v.quantity.unwrap());
        Ok(NewNumerableAmount {
            quantity: quantity.try_into().unwrap(),
            unit_value,
            ccy,
        })
    }
}

struct NewDividendAmount {
    ex_dividend_date: chrono::NaiveDate,
    _ex_dividend_snapshot_pk: i64,
    payout: NewNumerableAmount,
}

impl NewDividendAmount {
    pub fn get_amount(&self) -> (BigDecimal, &str) {
        self.payout.get_amount()
    }
}

enum NewMovementAmount {
    NonNumerable((bigdecimal::BigDecimal, String)),
    Numerable(NewNumerableAmount),
    DividendAmount(NewDividendAmount),
}

impl NewMovementAmount {
    pub fn get_amount(&self) -> (BigDecimal, &str) {
        match self {
            NewMovementAmount::NonNumerable(amount) => (amount.0.clone(), &amount.1),
            NewMovementAmount::Numerable(new_numerable_amount) => new_numerable_amount.get_amount(),
            NewMovementAmount::DividendAmount(new_dividend_amount) => new_dividend_amount.get_amount(),
        }
    }
}

impl TryFrom<crate::protos::finances_app_models::money_amount::NonNumerable> for NewMovementAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::money_amount::NonNumerable) -> Result<Self, Self::Error> {
        let amount = google_type::Money(v.amount.unwrap());
        let (amount, ccy): (BigDecimal, String) = amount.into();
        Ok(NewMovementAmount::NonNumerable((amount, ccy)))
    }
}

impl TryFrom<crate::protos::finances_app_models::money_amount::Numerable> for NewMovementAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::money_amount::Numerable) -> Result<Self, Self::Error> {
        Ok(NewMovementAmount::Numerable(v.try_into()?))
    }
}

impl TryFrom<crate::protos::finances_app_models::new_movement::DividendAmount> for NewMovementAmount {
    type Error = String;

    fn try_from(v: crate::protos::finances_app_models::new_movement::DividendAmount) -> Result<Self, Self::Error> {
        let ex_dividend_date: chrono::NaiveDate = {
            let date = google_type::Date(v.ex_dividend_date.expect("ex_dividend_date is requried"));
            date.into()
        };

        Ok(NewMovementAmount::DividendAmount(NewDividendAmount {
            ex_dividend_date,
            _ex_dividend_snapshot_pk: v.ex_dividend_snapshot_pk,
            payout: v.payout.expect("payout is required").try_into()?,
        }))
    }
}

impl TryFrom<crate::protos::finances_app_models::NewMovement> for NewMovement {
    type Error = String;

    fn try_from(proto: crate::protos::finances_app_models::NewMovement) -> Result<Self, Self::Error> {
        let date_value: chrono::NaiveDate = {
            let date = google_type::Date(proto.date_value.expect("date_value is requried"));
            date.into()
        };

        let new_movement_type: NewMovementAmount = match proto.amount.expect("amount is required") {
            crate::protos::finances_app_models::new_movement::Amount::NonNumerable(amount) => amount.try_into()?,
            crate::protos::finances_app_models::new_movement::Amount::Numerable(amount) => amount.try_into()?,
            crate::protos::finances_app_models::new_movement::Amount::Dividend(amount) => amount.try_into()?,
        };

        let fx: Option<BigDecimal> = proto
            .fx
            .map(|v| {
                let rate = google_type::Decimal(v.fx.expect("Rate is required in the FX"));
                rate.try_into()
            })
            .transpose()
            .map_err(|e| format!("Cannot convert deciaml into BigDecimal: {e}"))?;

        Ok(Self {
            r#type: new_movement_type,
            account_pk: proto.account_pk,
            date_value,
            movement_type_pk: proto.movement_type_pk,
            fx,
        })
    }
}
