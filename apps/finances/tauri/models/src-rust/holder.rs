use super::AppModel;
use prost::Message;

pub struct Holder(crate::protos::Holder);

impl From<finances_accounts::models::AccountHolder> for Holder {
    fn from(value: finances_accounts::models::AccountHolder) -> Self {
        Self(crate::protos::Holder {
            pk: value.id,
            name: value.name,
            is_company: value.is_company,
            photo: value.photo,
        })
    }
}

impl Holder {
    pub(crate) fn inner_type(self) -> crate::protos::Holder {
        self.0
    }
}

impl AppModel for Holder {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
