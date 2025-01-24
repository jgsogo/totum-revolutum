// TODO: A different ProtoWrapper[Enum] to wrap enums instead of messages

// use proto_wrapper::ProtoWrapper;
#[repr(transparent)]
#[derive(Debug, PartialEq)]
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

    pub(crate) fn to_i32(&self) -> i32 {
        self.0.into()
    }
}

impl TryFrom<i32> for MovementDirection {
    type Error = crate::errors::ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == crate::protos::finances_app_models::MovementDirection::In as i32 => Ok(MovementDirection::r#in()),
            x if x == crate::protos::finances_app_models::MovementDirection::Out as i32 => Ok(MovementDirection::out()),
            _ => Err(crate::errors::ConversionError::InvalidMovementDirection(value)),
        }
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
        write!(f, "{}", self.0.as_str_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_i32() {
        assert_eq!(MovementDirection::r#in().to_i32(), 0i32);
        assert_eq!(MovementDirection::out().to_i32(), 1i32);
    }

    #[test]
    fn try_from() {
        let dir_in: MovementDirection = 0i32.try_into().unwrap();
        assert_eq!(dir_in, MovementDirection::r#in());

        let dir_out: MovementDirection = 1i32.try_into().unwrap();
        assert_eq!(dir_out, MovementDirection::out());

        let invalid: Result<MovementDirection, _> = 2i32.try_into();
        assert!(invalid.is_err());
    }
}
