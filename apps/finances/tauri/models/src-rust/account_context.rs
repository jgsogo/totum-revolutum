use super::{Account, Movement, OutgoingModel, Snapshot};
use prost::Message;

#[derive(Debug)]
pub struct AccountContext(crate::protos::AccountContext);

// We only need this for testing (for `call_it_proto`)
impl TryFrom<Vec<u8>> for AccountContext {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::AccountContext::decode(&*v)?))
    }
}

impl AccountContext {
    pub fn new(account: Account, movements: Vec<Movement>, snapshots: Vec<Snapshot>) -> Self {
        Self(crate::protos::AccountContext {
            account: Some(account.0),
            movements: movements.into_iter().map(|v| v.0).collect(),
            snapshots: snapshots.into_iter().map(|v| v.0).collect(),
        })
    }

    pub fn snapshots(&self) -> Vec<Snapshot> {
        self.0.snapshots.iter().map(|v| Snapshot(v.clone())).collect()
    }
}

impl OutgoingModel for AccountContext {
    fn as_message(&self) -> &impl Message {
        &self.0
    }
}
