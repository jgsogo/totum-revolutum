use super::{google_type, MoneyAmount};

pub struct Snapshot(pub(crate) crate::protos::finances_app_models::Snapshot);

impl Snapshot {
    pub fn new(pk: i64, date_value: google_type::Date, amount: MoneyAmount) -> Self {
        Self(crate::protos::finances_app_models::Snapshot {
            pk,
            date_value: Some(date_value.0),
            amount: Some(amount.0),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }
}
