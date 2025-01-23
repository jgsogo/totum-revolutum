use crate::errors::{Error, Result};
use crate::google_type;
use bigdecimal::BigDecimal;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MoneyAmountNumerable(crate::protos::finances_app_models::money_amount::Numerable);

impl MoneyAmountNumerable {
    pub fn new(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        Self(crate::protos::finances_app_models::money_amount::Numerable {
            unit_value: Some(unit_value.into()),
            quantity: Some(quantity.into()),
        })
    }

    pub fn unit_value(&self) -> Result<&google_type::Money> {
        self.0
            .unit_value
            .as_ref()
            .map(google_type::Money::new_ref)
            .ok_or(Error::MissingRequiredField("unit_value".to_string()))
    }

    pub fn quantity(&self) -> Result<&google_type::Decimal> {
        self.0
            .quantity
            .as_ref()
            .map(google_type::Decimal::new_ref)
            .ok_or(Error::MissingRequiredField("quantity".to_string()))
    }

    pub fn amount(&self) -> Result<google_type::Money> {
        let quantity: BigDecimal = self.quantity()?.try_into()?;
        let result = self.unit_value()? * &quantity;
        Ok(result?)
    }
}
