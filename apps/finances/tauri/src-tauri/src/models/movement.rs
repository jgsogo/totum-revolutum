use serde::{Deserialize, Serialize};

use super::NewAmount;

#[derive(Deserialize, Serialize, Debug)]
pub enum NewMovementType {
    NonNumerable,
    Numerable,
    Dividend,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewMovement {
    pub account_pk: i64,
    pub movement_type_pk: i64,
    pub date_value: String,
    pub fx: Option<f32>,
    pub r#type: NewMovementType,

    #[serde(flatten)]
    pub amount: NewAmount,

    // dividend data
    pub ex_dividend_date: Option<String>,
    pub ex_dividend_snapshot_pk: Option<i64>,
}
