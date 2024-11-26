use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Holder {
    // pub pk: i32,
    pub name: String,
    pub is_company: bool,
}

impl From<finances_accounts::models::AccountHolder> for Holder {
    fn from(value: finances_accounts::models::AccountHolder) -> Self {
        Self {
            name: value.name,
            is_company: value.is_company,
        }
    }
}
