// TODO: A different ProtoWrapper[Enum] to wrap enums instead of messages

use crate::errors::ConversionError;

// use proto_wrapper::ProtoWrapper;
#[repr(transparent)]
#[derive(Debug, PartialEq)]
// #[derive(ProtoWrapper)]
pub struct AccountCategory(crate::protos::finances_app_models::AccountCategory);

impl AccountCategory {
    /// Creates a new [`AccountCategory`] with the Other value.
    pub fn other() -> Self {
        Self(crate::protos::finances_app_models::AccountCategory::Other)
    }

    /// Creates a new [`AccountCategory`] with the Savings value.
    pub fn savings() -> Self {
        Self(crate::protos::finances_app_models::AccountCategory::Savings)
    }

    /// Creates a new [`AccountCategory`] with the Investment value.
    pub fn investment() -> Self {
        Self(crate::protos::finances_app_models::AccountCategory::Investment)
    }

    /// Creates a new [`AccountCategory`] with the Retirement value.
    pub fn retirement() -> Self {
        Self(crate::protos::finances_app_models::AccountCategory::Retirement)
    }

    pub(crate) fn to_i32(&self) -> i32 {
        self.0.into()
    }
}

impl TryFrom<i32> for AccountCategory {
    type Error = crate::errors::ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == crate::protos::finances_app_models::AccountCategory::Other as i32 => Ok(AccountCategory::other()),
            x if x == crate::protos::finances_app_models::AccountCategory::Savings as i32 => {
                Ok(AccountCategory::savings())
            }
            x if x == crate::protos::finances_app_models::AccountCategory::Investment as i32 => {
                Ok(AccountCategory::investment())
            }
            x if x == crate::protos::finances_app_models::AccountCategory::Retirement as i32 => {
                Ok(AccountCategory::retirement())
            }
            _ => Err(crate::errors::ConversionError::InvalidAccountCategory(value)),
        }
    }
}

impl std::fmt::Display for AccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_str_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_i32() {
        assert_eq!(AccountCategory::other().to_i32(), 0i32);
        assert_eq!(AccountCategory::savings().to_i32(), 1i32);
        assert_eq!(AccountCategory::investment().to_i32(), 2i32);
        assert_eq!(AccountCategory::retirement().to_i32(), 3i32);
    }

    #[test]
    fn try_from() {
        let cat: AccountCategory = 0i32.try_into().unwrap();
        assert_eq!(cat, AccountCategory::other());

        let cat: AccountCategory = 1i32.try_into().unwrap();
        assert_eq!(cat, AccountCategory::savings());

        let cat: AccountCategory = 2i32.try_into().unwrap();
        assert_eq!(cat, AccountCategory::investment());

        let cat: AccountCategory = 3i32.try_into().unwrap();
        assert_eq!(cat, AccountCategory::retirement());
    }
}
