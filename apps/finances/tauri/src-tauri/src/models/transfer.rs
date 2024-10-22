use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub pk: i32,
    pub description: String,
}

impl From<finances_db::models::Transfer> for Transfer {
    fn from(value: finances_db::models::Transfer) -> Self {
        Self {
            pk: value.id,
            description: value.description,
        }
    }
}
