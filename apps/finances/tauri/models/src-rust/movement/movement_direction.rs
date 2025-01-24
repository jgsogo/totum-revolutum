// TODO: A different ProtoWrapper[Enum] to wrap enums instead of messages

// use proto_wrapper::ProtoWrapper;
#[repr(transparent)]
// #[derive(ProtoWrapper)]
pub struct MovementDirection(crate::protos::finances_app_models::MovementDirection);

impl MovementDirection {
    /// Creates a new [`MovementDirection`] with the IN value.
    pub fn r#in() -> Self {
        Self(crate::protos::finances_app_models::MovementDirection::In)
    }

    /// Creates a new [`MovementDirection`] with the OUT value.
    pub fn out() -> Self {
        Self(crate::protos::finances_app_models::MovementDirection::Out)
    }
}

// impl ProtoWrapper<crate::protos::finances_app_models::MovementDirection> for MovementDirection {
//     fn as_proto(&self) -> &crate::protos::finances_app_models::MovementDirection {
//         &self.0
//     }
// }

// impl From<crate::protos::finances_app_models::MovementDirection> for MovementDirection {
//     fn from(value: crate::protos::finances_app_models::MovementDirection) -> Self {
//         Self(value)
//     }
// }

// impl From<MovementDirection> for crate::protos::finances_app_models::MovementDirection {
//     fn from(val: MovementDirection) -> Self {
//         val.0
//     }
// }

impl From<finances_accounts::fields::MovementDirection> for MovementDirection {
    fn from(value: finances_accounts::fields::MovementDirection) -> Self {
        match value {
            finances_accounts::fields::MovementDirection::In => {
                Self(crate::protos::finances_app_models::MovementDirection::In)
            }
            finances_accounts::fields::MovementDirection::Out => {
                Self(crate::protos::finances_app_models::MovementDirection::Out)
            }
        }
    }
}

impl From<MovementDirection> for finances_accounts::fields::MovementDirection {
    fn from(val: MovementDirection) -> Self {
        match val.0 {
            crate::protos::finances_app_models::MovementDirection::In => {
                finances_accounts::fields::MovementDirection::In
            }
            crate::protos::finances_app_models::MovementDirection::Out => {
                finances_accounts::fields::MovementDirection::Out
            }
        }
    }
}

impl std::fmt::Display for MovementDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            crate::protos::finances_app_models::MovementDirection::In => write!(f, "In"),
            crate::protos::finances_app_models::MovementDirection::Out => write!(f, "Out"),
        }
    }
}
