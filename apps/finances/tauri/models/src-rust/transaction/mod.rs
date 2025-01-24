mod transaction_group;

pub use transaction_group::TransactionGroup;

use crate::Movement;
use proto_wrapper::ProtoWrapper;

#[repr(transparent)]
#[derive(ProtoWrapper)]
pub struct Transaction(crate::protos::finances_app_models::Transaction);

impl Transaction {
    pub fn new(
        pk: i64,
        name: String,
        description: Option<String>,
        group: TransactionGroup,
        movements_from: Vec<Movement>,
        movements_to: Vec<Movement>,
    ) -> Self {
        Self(crate::protos::finances_app_models::Transaction {
            pk,
            name,
            description,
            group: Some(group.into()),
            movements_from: movements_from.into_iter().map(|v| v.into()).collect(),
            movements_to: movements_to.into_iter().map(|v| v.into()).collect(),
        })
    }
    pub fn pk(&self) -> &i64 {
        &self.0.pk
    }

    pub fn name(&self) -> &str {
        &self.0.name.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }
}
