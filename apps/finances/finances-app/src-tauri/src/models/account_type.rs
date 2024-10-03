use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct AccountType {
    // pub pk: i32,
    pub name: String,
}

impl From<finances_db::models::AccountType> for AccountType {
    fn from(value: finances_db::models::AccountType) -> Self {
        Self { name: value.name }
    }
}
