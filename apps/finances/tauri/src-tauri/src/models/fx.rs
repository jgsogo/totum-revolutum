use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fx {
    // pub pk: i32,
    pub foreign: String,
    pub local: String,
    pub rate: f32,
    pub date_value: String,
}

impl From<finances_accounts::models::Fx> for Fx {
    fn from(value: finances_accounts::models::Fx) -> Self {
        Self {
            foreign: value.foreign,
            local: value.local,
            rate: value.rate.to_f32().unwrap(),
            date_value: value.date_value.format("%Y-%m-%d").to_string(),
        }
    }
}
