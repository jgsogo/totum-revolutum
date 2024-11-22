use bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::models::{Account, Fx, MovementType, Transaction};

type MovementAndRelatedData = (
    finances_accounts::models::Movement,
    // Option<finances_db::models::Fx>,
    finances_accounts::models::Transaction,
    finances_accounts::models::MovementType,
);

#[derive(Serialize, Deserialize, Debug)]
pub enum MovementAccount {
    Id(i32),
    Account(Account),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Movement {
    // pub pk: i32,
    pub amount: f32,
    pub quantity: Option<i32>,
    pub unit_value: Option<f32>,
    pub direction: i32,
    pub date: String,
    pub date_value: String,
    pub account: MovementAccount,
    pub fx: Option<Fx>,
    pub transaction: Transaction,
    pub r#type: MovementType,
}

impl From<MovementAndRelatedData> for Movement {
    fn from(value: MovementAndRelatedData) -> Self {
        let (movement, transaction, movement_type) = value;
        Self {
            amount: movement.amount.to_f32().unwrap(),
            quantity: movement.quantity,
            unit_value: movement.unit_value.map(|v| v.to_f32().unwrap()),
            direction: movement.direction,
            date: movement.date.format("%Y-%m-%d").to_string(),
            date_value: movement.date_value.format("%Y-%m-%d").to_string(),
            account: MovementAccount::Id(movement.account_id),
            fx: None,
            transaction: transaction.into(),
            r#type: movement_type.into(),
        }
    }
}
