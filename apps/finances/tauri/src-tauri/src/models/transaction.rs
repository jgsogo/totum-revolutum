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
