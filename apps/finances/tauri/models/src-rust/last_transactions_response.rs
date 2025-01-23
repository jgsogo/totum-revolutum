use super::{OutgoingModel, Transaction};
use prost::Message;

#[derive(Debug)]
pub struct LastTransactionsResponse(crate::protos::finances_app_models::LastTransactionsResponse);

impl LastTransactionsResponse {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self(crate::protos::finances_app_models::LastTransactionsResponse {
            transactions: transactions.into_iter().map(|v| v.0).collect(),
        })
    }
}

impl OutgoingModel for LastTransactionsResponse {
    fn encode_to_vec(&self) -> Vec<u8> {
        self.0.encode_to_vec()
    }
}
