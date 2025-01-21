use super::{google_type, Fx, MoneyAmount};

pub struct Movement(pub(crate) crate::protos::finances_app_models::Movement);

#[derive(PartialEq)]
pub enum MovementDirection {
    In,
    Out,
}

impl From<finances_accounts::fields::MovementDirection> for MovementDirection {
    fn from(value: finances_accounts::fields::MovementDirection) -> Self {
        match value {
            finances_accounts::fields::MovementDirection::In => MovementDirection::In,
            finances_accounts::fields::MovementDirection::Out => MovementDirection::Out,
        }
    }
}

impl From<crate::protos::finances_app_models::MovementDirection> for MovementDirection {
    fn from(value: crate::protos::finances_app_models::MovementDirection) -> Self {
        match value {
            crate::protos::finances_app_models::MovementDirection::In => MovementDirection::In,
            crate::protos::finances_app_models::MovementDirection::Out => MovementDirection::Out,
        }
    }
}

impl From<MovementDirection> for crate::protos::finances_app_models::MovementDirection {
    fn from(val: MovementDirection) -> Self {
        match val {
            MovementDirection::In => crate::protos::finances_app_models::MovementDirection::In,
            MovementDirection::Out => crate::protos::finances_app_models::MovementDirection::Out,
        }
    }
}

impl From<MovementDirection> for finances_accounts::fields::MovementDirection {
    fn from(val: MovementDirection) -> Self {
        match val {
            MovementDirection::In => finances_accounts::fields::MovementDirection::In,
            MovementDirection::Out => finances_accounts::fields::MovementDirection::Out,
        }
    }
}

impl TryFrom<i32> for MovementDirection {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::In),
            1 => Ok(Self::Out),
            _ => Err(format!("Cannot convert i32 '{value}' into MovementDirection")),
        }
    }
}

impl std::fmt::Display for MovementDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovementDirection::In => write!(f, "In"),
            MovementDirection::Out => write!(f, "Out"),
        }
    }
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
        let direction: crate::protos::finances_app_models::MovementDirection = direction.into();
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
