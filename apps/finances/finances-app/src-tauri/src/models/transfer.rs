use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Transfer {
    // pub pk: i32,
    pub description: String,
}

impl From<finances_db::models::Transfer> for Transfer {
    fn from(value: finances_db::models::Transfer) -> Self {
        Self {
            description: value.description,
        }
    }
}
