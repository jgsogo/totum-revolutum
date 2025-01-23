
mod numerable;

use crate::traits::ProtoWrapper;

pub struct MoneyAmount(crate::protos::finances_app_models::MoneyAmount);

impl MoneyAmount {}

impl ProtoWrapper<crate::protos::finances_app_models::MoneyAmount> for MoneyAmount {
    fn as_proto(&self) -> &crate::protos::finances_app_models::MoneyAmount {
        &self.0
    }
}

impl From<crate::protos::finances_app_models::MoneyAmount> for MoneyAmount {
    fn from(value: crate::protos::finances_app_models::MoneyAmount) -> Self {
        Self(value)
    }
}

impl From<MoneyAmount> for crate::protos::finances_app_models::MoneyAmount {
    fn from(val: MoneyAmount) -> Self {
        val.0
    }
}
