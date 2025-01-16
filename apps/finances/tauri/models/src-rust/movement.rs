use super::{google_type, Fx, MoneyAmount};

pub struct Movement(pub(crate) crate::protos::Movement);

pub enum MovementDirection {
    In,
    Out,
}

impl Movement {
    pub fn new(
        pk: i64,
        date_value: google_type::Date,
        transaction_pk: i64,
        r#type: crate::protos::MovementType,
        direction: MovementDirection,
        amount: MoneyAmount,
        fx: Option<Fx>,
    ) -> Self {
        let direction = match direction {
            MovementDirection::In => crate::protos::MovementDirection::In,
            MovementDirection::Out => crate::protos::MovementDirection::Out,
        };
        Self(crate::protos::Movement {
            pk,
            date_value: Some(date_value.into()),
            transaction_pk,
            r#type: Some(r#type),
            direction: direction.into(),
            amount: Some(amount.into()),
            fx: fx.map(|v| v.0),
        })
    }
}
