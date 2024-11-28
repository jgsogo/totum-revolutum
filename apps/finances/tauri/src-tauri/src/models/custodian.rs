use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Custodian {
    pub pk: i64,
    pub name: String,
    pub photo: Option<String>,
}

impl From<finances_accounts::models::Custodian> for Custodian {
    fn from(value: finances_accounts::models::Custodian) -> Self {
        Self {
            pk: value.id,
            name: value.name,
            photo: value.photo,
        }
    }
}
