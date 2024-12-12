use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionGroup {
    pub pk: i64,
    pub name: String,
    pub description: Option<String>,
}

impl From<finances_accounts::models::TransactionGroup> for TransactionGroup {
    fn from(value: finances_accounts::models::TransactionGroup) -> Self {
        Self {
            pk: value.id,
            name: value.name,
            description: value.description,
        }
    }
}
