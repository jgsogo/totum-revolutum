use crate::{Error, Result};

use proto_wrapper::ProtoWrapper;

use crate::google_type;
use crate::MoneyAmount;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Snapshot(crate::protos::finances_app_models::Snapshot);

impl Snapshot {
    pub fn new(pk: i64, date_value: google_type::Date, amount: MoneyAmount) -> Self {
        Self(crate::protos::finances_app_models::Snapshot {
            pk,
            date_value: Some(date_value.into()),
            amount: Some(amount.into()),
        })
    }
    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn date_value(&self) -> Result<&google_type::Date> {
        self.0
            .date_value
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("date_value".to_string()))
    }

    pub fn amount(&self) -> Result<&MoneyAmount> {
        self.0
            .amount
            .as_ref()
            .map(MoneyAmount::new_ref)
            .ok_or(Error::MissingRequiredField("amount".to_string()))
    }
}
