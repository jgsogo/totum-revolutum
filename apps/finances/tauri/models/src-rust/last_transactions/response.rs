use crate::Transaction;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper, Debug)]
pub struct LastTransactionsResponse(crate::protos::finances_app_models::LastTransactionsResponse);

impl LastTransactionsResponse {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Self(crate::protos::finances_app_models::LastTransactionsResponse {
            transactions: transactions.into_iter().map(|v| v.into()).collect(),
        })
    }
}
