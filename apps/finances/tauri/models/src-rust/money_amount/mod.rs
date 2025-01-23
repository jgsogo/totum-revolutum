mod non_numerable;
mod numerable;
use crate::errors::{Error, Result};
use crate::google_type;

use proto_wrapper::ProtoWrapper;

use non_numerable::MoneyAmountNonNumerable;
use numerable::MoneyAmountNumerable;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MoneyAmount(crate::protos::finances_app_models::MoneyAmount);

impl MoneyAmount {
    pub fn new_non_numerable(amount: google_type::Money) -> Self {
        let non_numerable = MoneyAmountNonNumerable::new(amount);
        let amount = crate::protos::finances_app_models::money_amount::Amount::NonNumerable(non_numerable.into());
        Self(crate::protos::finances_app_models::MoneyAmount { amount: Some(amount) })
    }

    pub fn new_numerable(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        let numerable = MoneyAmountNumerable::new(unit_value, quantity);
        let amount = crate::protos::finances_app_models::money_amount::Amount::Numerable(numerable.into());
        Self(crate::protos::finances_app_models::MoneyAmount { amount: Some(amount) })
    }

    pub fn amount(&self) -> Result<google_type::Money> {
        match self
            .0
            .amount
            .as_ref()
            .ok_or(Error::MissingRequiredField("amount".to_string()))?
        {
            crate::protos::finances_app_models::money_amount::Amount::NonNumerable(non_numerable) => {
                MoneyAmountNonNumerable::new_ref(non_numerable).amount().cloned()
            }
            crate::protos::finances_app_models::money_amount::Amount::Numerable(numerable) => {
                MoneyAmountNumerable::new_ref(numerable).amount()
            }
        }
    }
}
