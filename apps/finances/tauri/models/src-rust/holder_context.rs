use super::{Account, Holder};

use crate::{Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct HolderContext(crate::protos::finances_app_models::HolderContext);

impl HolderContext {
    pub fn new(holder: Holder, accounts: Vec<Account>) -> Self {
        Self(crate::protos::finances_app_models::HolderContext {
            holder: Some(holder.into()),
            accounts: accounts.into_iter().map(|v| v.into()).collect(),
        })
    }

    pub fn holder(&self) -> Result<&Holder> {
        self.0
            .holder
            .as_ref()
            .map(Holder::new_ref)
            .ok_or(Error::MissingRequiredField("holder".to_string()))
    }

    pub fn accounts(&self) -> Vec<&Account> {
        self.0.accounts.iter().map(Account::new_ref).collect()
    }
}
