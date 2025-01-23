use super::google_type;

/// This type can be created from primitive types to construct a protobuf message
pub struct MovementAmount(pub(crate) crate::protos::finances_app_models::MovementAmount);

impl From<MovementAmount> for crate::protos::finances_app_models::MovementAmount {
    fn from(val: MovementAmount) -> Self {
        val.0
    }
}

impl MovementAmount {
    pub fn new_numerable(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        let amount = crate::protos::finances_app_models::money_amount::Numerable {
            quantity: Some(quantity.into()),
            unit_value: Some(unit_value.into()),
        };
        Self(crate::protos::finances_app_models::MovementAmount {
            amount: Some(crate::protos::finances_app_models::movement_amount::Amount::Numerable(
                amount,
            )),
        })
    }

    pub fn new_non_numerable(amount: google_type::Money) -> Self {
        let amount = crate::protos::finances_app_models::money_amount::NonNumerable {
            amount: Some(amount.into()),
        };
        Self(crate::protos::finances_app_models::MovementAmount {
            amount: Some(crate::protos::finances_app_models::movement_amount::Amount::NonNumerable(amount)),
        })
    }

    pub fn new_dividend(
        ex_dividend_date: google_type::Date,
        unit_value: google_type::Money,
        quantity: google_type::Decimal,
    ) -> Self {
        let payout = crate::protos::finances_app_models::money_amount::Numerable {
            quantity: Some(quantity.into()),
            unit_value: Some(unit_value.into()),
        };
        let amount = crate::protos::finances_app_models::movement_amount::Dividend {
            ex_dividend_date: Some(ex_dividend_date.into()),
            payout: Some(payout),
        };
        Self(crate::protos::finances_app_models::MovementAmount {
            amount: Some(crate::protos::finances_app_models::movement_amount::Amount::Dividend(
                amount,
            )),
        })
    }
}

/// This type can be created from proto message to get the data as primitive types
pub enum NewMovementAmount {
    NonNumerable(google_type::NewMoney),
    Numerable(NewNumerableAmount),
    DividendAmount(NewDividendAmount),
}

impl NewMovementAmount {
    pub fn get_amount(&self) -> google_type::NewMoney {
        match self {
            NewMovementAmount::NonNumerable(new_money) => new_money.clone(),
            NewMovementAmount::Numerable(new_numerable_amount) => new_numerable_amount.get_amount(),
            NewMovementAmount::DividendAmount(new_dividend_amount) => new_dividend_amount.get_amount(),
        }
    }
}

pub struct NewNumerableAmount {
    quantity: bigdecimal::BigDecimal,
    unit_value: google_type::NewMoney,
}

impl NewNumerableAmount {
    pub fn get_amount(&self) -> google_type::NewMoney {
        &self.unit_value * &self.quantity
    }
}

pub struct NewDividendAmount {
    ex_dividend_date: chrono::NaiveDate,
    _ex_dividend_snapshot_pk: i64,
    payout: NewNumerableAmount,
}

impl NewDividendAmount {
    pub fn get_amount(&self) -> google_type::NewMoney {
        self.payout.get_amount()
    }
}

impl TryFrom<crate::protos::finances_app_models::MovementAmount> for NewMovementAmount {
    type Error = String;

    fn try_from(value: crate::protos::finances_app_models::MovementAmount) -> Result<Self, Self::Error> {
        todo!()
    }
}
