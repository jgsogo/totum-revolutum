use crate::{Error, Result};

use proto_wrapper::ProtoWrapper;

use crate::google_type;
use crate::MoneyAmount;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug, Clone)]
pub struct Snapshot(crate::protos::finances_app_models::Snapshot);

impl Snapshot {
    pub fn new(pk: Option<i64>, date_value: google_type::Date, amount: MoneyAmount, account_pk: i64) -> Self {
        Self(crate::protos::finances_app_models::Snapshot {
            pk,
            date_value: Some(date_value.into()),
            amount: Some(amount.into()),
            account_pk,
        })
    }
    pub fn pk(&self) -> Option<&i64> {
        self.0.pk.as_ref()
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

    pub fn account_pk(&self) -> &i64 {
        &self.0.account_pk
    }
}
