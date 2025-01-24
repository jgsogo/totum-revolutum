use proto_wrapper::ProtoWrapper;

use super::AccountCategory;
use crate::Result;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct AccountType(crate::protos::finances_app_models::AccountType);

impl AccountType {
    pub fn new(pk: i64, name: String, breadcrumb: Option<Vec<String>>, category: AccountCategory) -> Self {
        Self(crate::protos::finances_app_models::AccountType {
            pk,
            name,
            breadcrumb: breadcrumb.unwrap_or_default(),
            category: category.to_i32(),
        })
    }

    pub fn pk(&self) -> i64 {
        self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn breadcrumb(&self) -> Option<&Vec<String>> {
        if self.0.breadcrumb.is_empty() {
            None
        } else {
            Some(&self.0.breadcrumb)
        }
    }

    pub fn category(&self) -> Result<AccountCategory> {
        Ok(self.0.category.try_into()?)
    }
}
