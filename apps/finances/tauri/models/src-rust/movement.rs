use super::{google_type, AppModel, Fx, MoneyAmount, MovementType};

pub struct Movement(crate::protos::Movement);

impl Movement {
    pub fn new(
        pk: i64,
        date_value: google_type::Date,
        transaction_pk: i64,
        r#type: MovementType,
        direction: crate::protos::MovementDirection,
        amount: MoneyAmount,
        fx: Fx,
    ) -> Self {
        Self(crate::protos::Movement {
            pk,
            date_value: Some(date_value.into()),
            transaction_pk,
            r#type: Some(r#type.inner_type()),
            direction: direction.into(),
            amount: Some(amount.into()),
            fx: Some(fx.inner_type()),
        })
    }
}

impl AppModel<crate::protos::Movement> for Movement {
    fn inner_type(self) -> crate::protos::Movement {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::Movement {
        &self.0
    }
}
