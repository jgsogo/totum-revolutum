use super::{google_type, MoneyAmount};

pub struct Snapshot(pub(crate) crate::protos::Snapshot);

impl Snapshot {
    pub fn new(pk: i64, date_value: google_type::Date, amount: MoneyAmount) -> Self {
        Self(crate::protos::Snapshot {
            pk,
            date_value: Some(date_value.into()),
            amount: Some(amount.into()),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }
}
