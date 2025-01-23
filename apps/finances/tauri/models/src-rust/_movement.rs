use super::{google_type, Fx, MovementAmount, MovementDirection};

pub struct Movement(pub(crate) crate::protos::finances_app_models::Movement);

impl Movement {
    pub fn new(
        pk: i64,
        date_value: google_type::Date,
        transaction_pk: i64,
        r#type: crate::protos::finances_app_models::MovementType,
        direction: MovementDirection,
        amount: MovementAmount,
        fx: Option<Fx>,
    ) -> Self {
        let direction: crate::protos::finances_app_models::MovementDirection = direction.into();
        Self(crate::protos::finances_app_models::Movement {
            pk,
            date_value: Some(date_value.0),
            transaction_pk,
            r#type: Some(r#type),
            direction: direction.into(),
            amount: Some(amount.into()),
            fx: fx.map(|v| v.0),
        })
    }
}
