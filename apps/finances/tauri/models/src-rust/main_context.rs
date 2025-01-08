use super::{AppModel, Holder};
use prost::Message;

pub struct MainContext(crate::protos::MainContext);

impl MainContext {
    pub fn new(holders: Vec<finances_accounts::models::AccountHolder>) -> Self {
        Self(crate::protos::MainContext {
            holders: holders
                .into_iter()
                .map(|v| {
                    let holder: Holder = v.into();
                    holder.inner_type()
                })
                .collect(),
        })
    }
}

impl AppModel for MainContext {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
