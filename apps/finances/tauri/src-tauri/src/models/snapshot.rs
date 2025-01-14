use super::NewAmount;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct NewSnapshot {
    pub account_pk: i64,
    pub date_value: String,
    #[serde(flatten)]
    pub amount: NewAmount,
}
