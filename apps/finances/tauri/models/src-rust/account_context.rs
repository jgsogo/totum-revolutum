use super::{Account, Movement, Snapshot};

use crate::{Error, Result};
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct AccountContext(crate::protos::finances_app_models::AccountContext);

impl AccountContext {
    pub fn new(account: Account, movements: Vec<Movement>, snapshots: Vec<Snapshot>) -> Self {
        Self(crate::protos::finances_app_models::AccountContext {
            account: Some(account.into()),
            movements: movements.into_iter().map(|v| v.into()).collect(),
            snapshots: snapshots.into_iter().map(|v| v.into()).collect(),
        })
    }

    pub fn account(&self) -> Result<&Account> {
        self.0
            .account
            .as_ref()
            .map(Account::new_ref)
            .ok_or(Error::MissingRequiredField("account".to_string()))
    }

    pub fn snapshots(&self) -> Vec<&Snapshot> {
        self.0.snapshots.iter().map(Snapshot::new_ref).collect()
    }

    pub fn movements(&self) -> Vec<&Movement> {
        self.0.movements.iter().map(Movement::new_ref).collect()
    }
}
