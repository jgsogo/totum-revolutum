mod transaction_group;

pub use transaction_group::TransactionGroup;

use crate::{Error, Movement, Result};
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
        self.0.name.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.0.description.as_deref()
    }

    pub fn group(&self) -> Option<&TransactionGroup> {
        self.0.group.as_ref().map(TransactionGroup::new_ref)
    }

    pub fn movements_from(&self) -> impl Iterator<Item = &Movement> {
        self.0.movements_from.iter().map(Movement::new_ref)
    }

    pub fn movements_to(&self) -> impl Iterator<Item = &Movement> {
        self.0.movements_to.iter().map(Movement::new_ref)
    }
}
