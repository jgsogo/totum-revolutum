use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub account_id: i32,
    pub amount: f32,
    pub quantity: Option<i32>,
    pub unit_value: Option<f32>,
    pub date_value: String,
}

impl From<finances_accounts::models::Snapshot> for Snapshot {
    fn from(value: finances_accounts::models::Snapshot) -> Self {
        Self {
            account_id: value.account_id,
            amount: value.amount.to_f32().unwrap(),
            quantity: value.quantity,
            unit_value: value.unit_value.and_then(|v| v.to_f32()),
            date_value: value.date_value.format("%Y-%m-%d").to_string(),
        }
    }
}
