use super::{Account, Holder, OutgoingModel};
use prost::Message;
pub struct HolderContext(crate::protos::finances_app_models::HolderContext);

impl TryFrom<Vec<u8>> for HolderContext {
    type Error = prost::DecodeError;

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(crate::protos::finances_app_models::HolderContext::decode(&*v)?))
    }
}

impl HolderContext {
    pub fn new(holder: finances_accounts::models::AccountHolder, accounts: Vec<Account>) -> Self {
        let holder: Holder = holder.into();
        Self(crate::protos::finances_app_models::HolderContext {
            holder: Some(holder.0),
            accounts: accounts.into_iter().map(|v| v.0).collect(),
        })
    }
}

impl OutgoingModel for HolderContext {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
