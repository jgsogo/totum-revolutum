use super::{Account, AppModel, Movement, Snapshot};
use prost::Message;

#[derive(Debug)]
pub struct AccountContext(crate::protos::AccountContext);

impl TryFrom<Vec<u8>> for AccountContext {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::AccountContext::decode(&*v)?))
    }
}

impl AccountContext {
    pub fn new(account: Account, movements: Vec<Movement>, snapshots: Vec<Snapshot>) -> Self {
        Self(crate::protos::AccountContext {
            account: Some(account.inner_type()),
            movements: movements.into_iter().map(|v| v.inner_type()).collect(),
            snapshots: snapshots.into_iter().map(|v| v.inner_type()).collect(),
        })
    }

    pub fn snapshots(&self) -> Vec<Snapshot> {
        self.0.snapshots.iter().map(|v| Snapshot(v.clone())).collect()
    }
}

impl AppModel<crate::protos::AccountContext> for AccountContext {
    fn inner_type(self) -> crate::protos::AccountContext {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::AccountContext {
        &self.0
    }
}
