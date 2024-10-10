use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Holder {
    // pub pk: i32,
    pub name: String,
}

impl From<finances_db::models::AccountHolder> for Holder {
    fn from(value: finances_db::models::AccountHolder) -> Self {
        Self { name: value.name }
    }
}
