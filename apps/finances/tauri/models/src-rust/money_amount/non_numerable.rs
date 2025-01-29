use crate::errors::{Error, Result};
use crate::google_type;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct MoneyAmountNonNumerable(crate::protos::finances_app_models::money_amount::NonNumerable);

impl MoneyAmountNonNumerable {
    pub fn new(amount: google_type::Money) -> Self {
        Self(crate::protos::finances_app_models::money_amount::NonNumerable {
            amount: Some(amount.into()),
        })
    }

    pub fn amount(&self) -> Result<&google_type::Money> {
        self.0
            .amount
            .as_ref()
            .map(google_type::Money::new_ref)
            .ok_or(Error::MissingRequiredField("amount".to_string()))
    }
}
