use super::{google_type, AppModel, MoneyAmount};

pub struct Snapshot(crate::protos::Snapshot);

impl Snapshot {
    pub fn new(pk: i64, date_value: google_type::Date, amount: MoneyAmount) -> Self {
        Self(crate::protos::Snapshot {
            pk,
            date_value: Some(date_value.into()),
            amount: Some(amount.into()),
        })
    }
}

impl AppModel<crate::protos::Snapshot> for Snapshot {
    fn inner_type(self) -> crate::protos::Snapshot {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::Snapshot {
        &self.0
    }
}
