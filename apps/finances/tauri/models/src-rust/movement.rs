use super::{google_type, Fx, MoneyAmount};

pub struct Movement(pub(crate) crate::protos::finances_app_models::Movement);

pub enum MovementDirection {
    In,
    Out,
}

impl Movement {
    pub fn new(
        pk: i64,
        date_value: google_type::Date,
        transaction_pk: i64,
        r#type: crate::protos::finances_app_models::MovementType,
        direction: MovementDirection,
        amount: MoneyAmount,
        fx: Option<Fx>,
    ) -> Self {
        let direction = match direction {
            MovementDirection::In => crate::protos::finances_app_models::MovementDirection::In,
            MovementDirection::Out => crate::protos::finances_app_models::MovementDirection::Out,
        };
        Self(crate::protos::finances_app_models::Movement {
            pk,
            date_value: Some(date_value.0),
            transaction_pk,
            r#type: Some(r#type),
            direction: direction.into(),
            amount: Some(amount.0),
            fx: fx.map(|v| v.0),
        })
    }
}
