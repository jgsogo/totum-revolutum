use crate::google_type;
use crate::money_amount::{MoneyAmountNonNumerable, MoneyAmountNumerable};
use crate::{Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MovementAmountDividend(crate::protos::finances_app_models::movement_amount::Dividend);

impl MovementAmountDividend {
    pub fn new(
        ex_dividend_date: google_type::Date,
        unit_value: google_type::Money,
        quantity: google_type::Decimal,
    ) -> Self {
        let payout = MoneyAmountNumerable::new(unit_value, quantity);
        Self(crate::protos::finances_app_models::movement_amount::Dividend {
            ex_dividend_date: Some(ex_dividend_date.into()),
            payout: Some(payout.into()),
        })
    }

    pub fn ex_dividend_date(&self) -> Result<&google_type::Date> {
        self.0
            .ex_dividend_date
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("ex_dividend_date".to_string()))
    }

    pub fn payout(&self) -> Result<&MoneyAmountNumerable> {
        self.0
            .payout
            .as_ref()
            .map(MoneyAmountNumerable::new_ref)
            .ok_or(Error::MissingRequiredField("payout".to_string()))
    }
}

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MovementAmount(crate::protos::finances_app_models::MovementAmount);

impl MovementAmount {
    pub fn new_non_numerable(amount: google_type::Money) -> Self {
        let non_numerable = MoneyAmountNonNumerable::new(amount);
        let amount = crate::protos::finances_app_models::movement_amount::Amount::NonNumerable(non_numerable.into());
        Self(crate::protos::finances_app_models::MovementAmount { amount: Some(amount) })
    }

    pub fn new_numerable(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        let numerable = MoneyAmountNumerable::new(unit_value, quantity);
        let amount = crate::protos::finances_app_models::movement_amount::Amount::Numerable(numerable.into());
        Self(crate::protos::finances_app_models::MovementAmount { amount: Some(amount) })
    }

    pub fn new_dividend(
        ex_dividend_date: google_type::Date,
        unit_value: google_type::Money,
        quantity: google_type::Decimal,
    ) -> Self {
        let dividend = MovementAmountDividend::new(ex_dividend_date, unit_value, quantity);
        let amount = crate::protos::finances_app_models::movement_amount::Amount::Dividend(dividend.into());
        Self(crate::protos::finances_app_models::MovementAmount { amount: Some(amount) })
    }

    pub fn amount(&self) -> Result<google_type::Money> {
        match self
            .0
            .amount
            .as_ref()
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
        {
            crate::protos::finances_app_models::movement_amount::Amount::NonNumerable(non_numerable) => {
                MoneyAmountNonNumerable::new_ref(non_numerable).amount().cloned()
            }
            crate::protos::finances_app_models::movement_amount::Amount::Numerable(numerable) => {
                MoneyAmountNumerable::new_ref(numerable).amount()
            }
            crate::protos::finances_app_models::movement_amount::Amount::Dividend(dividend) => {
                MovementAmountDividend::new_ref(dividend).payout()?.amount()
            }
        }
    }

    pub fn as_numerable(&self) -> Result<Option<&MoneyAmountNumerable>> {
        match self
            .0
            .amount
            .as_ref()
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
        {
            crate::protos::finances_app_models::movement_amount::Amount::Numerable(money_amount) => {
                Ok(Some(MoneyAmountNumerable::new_ref(money_amount)))
            }
            _ => Ok(None),
        }
    }

    pub fn as_non_numerable(&self) -> Result<Option<&MoneyAmountNonNumerable>> {
        match self
            .0
            .amount
            .as_ref()
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
        {
            crate::protos::finances_app_models::movement_amount::Amount::NonNumerable(money_amount) => {
                Ok(Some(MoneyAmountNonNumerable::new_ref(money_amount)))
            }
            _ => Ok(None),
        }
    }

    pub fn as_dividend(&self) -> Result<Option<&MovementAmountDividend>> {
        match self
            .0
            .amount
            .as_ref()
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
        {
            crate::protos::finances_app_models::movement_amount::Amount::Dividend(money_amount) => {
                Ok(Some(MovementAmountDividend::new_ref(money_amount)))
            }
            _ => Ok(None),
        }
    }
}
