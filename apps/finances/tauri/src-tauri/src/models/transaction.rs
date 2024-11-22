use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub pk: i32,
    pub description: String,
}

impl From<finances_accounts::models::Transaction> for Transaction {
    fn from(value: finances_accounts::models::Transaction) -> Self {
        Self {
            pk: value.id,
            description: value.description,
        }
    }
}
