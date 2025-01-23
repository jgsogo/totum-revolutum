use crate::errors::{Error, Result};
use crate::google_type;
use crate::traits::ProtoWrapper;

pub struct MoneyAmountNumerable(crate::protos::finances_app_models::money_amount::Numerable);

impl MoneyAmountNumerable {
    pub fn new(unit_value: google_type::Money, quantity: google_type::Decimal) -> Self {
        Self(crate::protos::finances_app_models::money_amount::Numerable {
            unit_value: Some(unit_value.into()),
            quantity: Some(quantity.into()),
        })
    }

    pub fn unit_value(&self) -> Result<google_type::MoneyRef> {
        let unit_value = self
            .0
            .unit_value
            .as_ref()
            .ok_or(Error::MissingRequiredField("unit_value".to_string()))?;
        Ok(unit_value.into())
    }
}

impl ProtoWrapper<crate::protos::finances_app_models::money_amount::Numerable> for MoneyAmountNumerable {
    fn as_proto(&self) -> &crate::protos::finances_app_models::money_amount::Numerable {
        &self.0
    }
}

impl From<crate::protos::finances_app_models::money_amount::Numerable> for MoneyAmountNumerable {
    fn from(value: crate::protos::finances_app_models::money_amount::Numerable) -> Self {
        Self(value)
    }
}

impl From<MoneyAmountNumerable> for crate::protos::finances_app_models::money_amount::Numerable {
    fn from(val: MoneyAmountNumerable) -> Self {
        val.0
    }
}
