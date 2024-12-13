use super::NewAmount;
use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub account_id: i64,
    pub amount: f32,
    pub date_value: String,

    pub quantity: Option<f32>,
    pub unit_value: Option<f32>,
}

impl From<finances_accounts::models::Snapshot> for Snapshot {
    fn from(value: finances_accounts::models::Snapshot) -> Self {
        Self {
            account_id: value.account_id,
            amount: value.amount.to_f32().unwrap(),
            date_value: value.date_value.format("%Y-%m-%d").to_string(),
            quantity: None,
            unit_value: None,
        }
    }
}

impl From<finances_investments::models::SnapshotNumerable> for Snapshot {
    fn from(value: finances_investments::models::SnapshotNumerable) -> Self {
        Self {
            account_id: value.snapshot.account_id,
            amount: value.snapshot.amount.to_f32().unwrap(),
            date_value: value.snapshot.date_value.format("%Y-%m-%d").to_string(),
            quantity: Some(value.snapshot_numerable.quantity.to_f32().unwrap()),
            unit_value: Some(value.snapshot_numerable.unit_value.to_f32().unwrap()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewSnapshot {
    pub account_pk: i64,
    pub date_value: String,
    #[serde(flatten)]
    pub amount: NewAmount,
}
