use super::{Account, Holder, OutgoingModel};
use prost::Message;
pub struct HolderContext(crate::protos::HolderContext);

impl TryFrom<Vec<u8>> for HolderContext {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::HolderContext::decode(&*v)?))
    }
}

impl HolderContext {
    pub fn new(holder: finances_accounts::models::AccountHolder, accounts: Vec<Account>) -> Self {
        let holder: Holder = holder.into();
        Self(crate::protos::HolderContext {
            holder: Some(holder.0),
            accounts: accounts.into_iter().map(|v| v.0).collect(),
        })
    }
}

impl OutgoingModel for HolderContext {
    fn as_message(&self) -> &impl Message {
        &self.0
    }
}
