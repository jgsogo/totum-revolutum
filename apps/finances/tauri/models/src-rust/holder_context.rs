use super::{Account, AppModel, Holder};
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
            holder: Some(holder.inner_type()),
            accounts: accounts.into_iter().map(|v| v.inner_type()).collect(),
        })
    }
}

impl AppModel<crate::protos::HolderContext> for HolderContext {
    fn inner_type(self) -> crate::protos::HolderContext {
        self.0
    }

    fn inner_type_ref(&self) -> &crate::protos::HolderContext {
        &self.0
    }
}
