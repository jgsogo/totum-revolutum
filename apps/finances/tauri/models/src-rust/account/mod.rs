mod account_category;
mod account_type;

pub use account_category::AccountCategory;
pub use account_type::AccountType;

use crate::{google_type::CurrencyCode, Custodian, Error, Result};

use proto_wrapper::ProtoWrapper;

use crate::google_type;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Account(crate::protos::finances_app_models::Account);

impl Account {
    pub fn new(
        pk: i64,
        name: String,
        custodian: Custodian,
        r#type: AccountType,
        currency_code: CurrencyCode,
        identifier: Option<String>,
        description: Option<String>,
        open: google_type::Date,
        holder_owns_money: bool,
        is_numerable: bool,
    ) -> Self {
        Self(crate::protos::finances_app_models::Account {
            pk,
            name,
            custodian: Some(custodian.into()),
            r#type: Some(r#type.into()),
            currency_code: currency_code.to_string(),
            identifier,
            description,
            open: Some(open.into()),
            holder_owns_money,
            is_numerable,
        })
    }

    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn custodian(&self) -> Result<&Custodian> {
        self.0
            .custodian
            .as_ref()
            .map(Custodian::new_ref)
            .ok_or(Error::MissingRequiredField("custodian".to_string()))
    }

    pub fn r#type(&self) -> Result<&AccountType> {
        self.0
            .r#type
            .as_ref()
            .map(AccountType::new_ref)
            .ok_or(Error::MissingRequiredField("type".to_string()))
    }

    pub fn currency_code(&self) -> Result<CurrencyCode> {
        Ok(CurrencyCode::new(&self.0.currency_code)?)
    }

    pub fn identifier(&self) -> Option<&str> {
        self.0.identifier.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    pub fn open(&self) -> Result<&google_type::Date> {
        self.0
            .open
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("open".to_string()))
    }

    pub fn holder_owns_money(&self) -> bool {
        self.0.holder_owns_money
    }

    pub fn is_numerable(&self) -> bool {
        self.0.is_numerable
    }
}
