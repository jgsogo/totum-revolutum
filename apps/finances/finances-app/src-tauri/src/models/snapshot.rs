use bigdecimal::ToPrimitive;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Snapshot {
    pub amount: f32,
    pub quantity: Option<i32>,
    pub unit_value: Option<f32>,
    pub date_value: String,
}

impl From<finances_db::models::Snapshot> for Snapshot {
    fn from(value: finances_db::models::Snapshot) -> Self {
        Self {
            amount: value.amount.to_f32().unwrap(),
            quantity: value.quantity,
            unit_value: value.unit_value.and_then(|v| v.to_f32()),
            date_value: value.date_value.format("%Y-%m-%d").to_string(),
        }
    }
}
