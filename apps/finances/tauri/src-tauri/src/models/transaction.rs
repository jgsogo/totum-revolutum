use crate::models::movement::NewMovement;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub pk: i64,
    pub name: String,
    pub description: Option<String>,
}

impl From<finances_accounts::models::Transaction> for Transaction {
    fn from(value: finances_accounts::models::Transaction) -> Self {
        Self {
            pk: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewTransaction {
    pub name: String,
    pub description: Option<String>,
    pub transaction_group_pk: Option<i64>,
    pub movements_from: Vec<NewMovement>,
    pub movements_to: Vec<NewMovement>,
}
