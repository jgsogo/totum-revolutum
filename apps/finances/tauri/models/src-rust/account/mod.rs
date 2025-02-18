mod account_category;
mod account_type;

pub use account_category::AccountCategory;
pub use account_type::AccountType;

use crate::{google_type::CurrencyCode, Custodian, Error, Holder, Result, Snapshot};

use proto_wrapper::ProtoWrapper;

use crate::google_type;

#[repr(transparent)]
#[derive(ProtoWrapper, Clone)]
pub struct Account(crate::protos::finances_app_models::Account);

impl Account {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pk: i64,
        name: String,
        custodian: Custodian,
        r#type: AccountType,
        currency_code: CurrencyCode,
        identifier: Option<String>,
        description: Option<String>,
        open: google_type::Date,
        close: Option<google_type::Date>,
        holders: Vec<Holder>,
        is_numerable: bool,
        last_snapshot: Option<Snapshot>,
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
            close: close.map(|v| v.into()),
            holders: holders.into_iter().map(|v| v.into()).collect(),
            is_numerable,
            last_snapshot: last_snapshot.map(|v| v.into()),
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

    pub fn close(&self) -> Result<&google_type::Date> {
        self.0
            .close
            .as_ref()
            .map(google_type::Date::new_ref)
            .ok_or(Error::MissingRequiredField("close".to_string()))
    }

    pub fn holders(&self) -> Vec<&Holder> {
        self.0.holders.iter().map(Holder::new_ref).collect()
    }

    pub fn is_numerable(&self) -> bool {
        self.0.is_numerable
    }

    pub fn last_snapshot(&self) -> Result<&Snapshot> {
        self.0
            .last_snapshot
            .as_ref()
            .map(Snapshot::new_ref)
            .ok_or(Error::MissingRequiredField("last_snapshot".to_string()))
    }
}
